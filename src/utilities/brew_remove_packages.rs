use crate::constants::PROGRAM;
use crate::models::BrewPackage;
use crate::tui::{ProgressState, ProgressTracker};
use crate::webhook::PackageResult;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

pub fn remove_packages(packages: &[BrewPackage], _parallel: bool) -> Vec<PackageResult> {
    let package_names: Vec<String> = packages.iter().map(|p| p.name.clone()).collect();

    let mut tracker = match ProgressTracker::new(package_names) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to initialize TUI: {}", e);
            return vec![];
        }
    };

    let packages_arc = Arc::new(Mutex::new(packages.to_owned()));
    let tracker_packages = tracker.get_packages();
    let tracker_packages_for_result = Arc::clone(&tracker_packages);
    let cancelled = Arc::new(AtomicBool::new(false));

    let remove_threads: Vec<_> = {
        let packages = packages_arc.lock().unwrap();
        
        packages.iter().enumerate().map(|(index, package)| {
            let package = package.clone();
            let tracker_packages = Arc::clone(&tracker_packages);
            let cancelled = Arc::clone(&cancelled);
            
            thread::spawn(move || {
                if cancelled.load(Ordering::Relaxed) {
                    return;
                }
                
                if let Ok(mut tracked) = tracker_packages.lock() {
                    if let Some(p) = tracked.get_mut(index) {
                        p.state = ProgressState::Removing;
                        p.progress = 10;
                        p.message = "Removing...".to_string();
                    }
                }

                thread::sleep(Duration::from_millis(200));

                let mut command = Command::new(PROGRAM);
                command
                    .arg("remove")
                    .arg("-f")
                    .arg(&package.name)
                    .stdin(Stdio::null())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped());

                let mut child = match command.spawn() {
                    Ok(c) => c,
                    Err(e) => {
                        if let Ok(mut tracked) = tracker_packages.lock() {
                            if let Some(p) = tracked.get_mut(index) {
                                p.state = ProgressState::Failed;
                                p.progress = 0;
                                p.message = format!("Error: {}", e);
                            }
                        }
                        return;
                    }
                };

                let stdout = match child.stdout.take() {
                    Some(s) => s,
                    None => {
                        if let Ok(mut tracked) = tracker_packages.lock() {
                            if let Some(p) = tracked.get_mut(index) {
                                p.state = ProgressState::Failed;
                                p.message = "Failed to capture stdout".to_string();
                            }
                        }
                        return;
                    }
                };
                let stderr = match child.stderr.take() {
                    Some(s) => s,
                    None => {
                        if let Ok(mut tracked) = tracker_packages.lock() {
                            if let Some(p) = tracked.get_mut(index) {
                                p.state = ProgressState::Failed;
                                p.message = "Failed to capture stderr".to_string();
                            }
                        }
                        return;
                    }
                };
                let tracker_packages_clone = Arc::clone(&tracker_packages);
                
                let stdout_thread = thread::spawn(move || {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines().map_while(Result::ok) {
                        if let Ok(mut tracked) = tracker_packages_clone.try_lock() {
                                if let Some(p) = tracked.get_mut(index) {
                                    p.progress = 50;
                                    if !line.trim().is_empty() && line.len() < 50 {
                                        p.message = line.trim().to_string();
                                    }
                                }
                            }
                        }
                });

                let tracker_packages_clone = Arc::clone(&tracker_packages);
                let stderr_thread = thread::spawn(move || {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines().map_while(Result::ok) {
                        if !line.trim().is_empty() && line.len() < 50 {
                                if let Ok(mut tracked) = tracker_packages_clone.try_lock() {
                                    if let Some(p) = tracked.get_mut(index) {
                                        p.message = line.trim().to_string();
                                    }
                                }
                            }
                        }
                });

                #[allow(unused_assignments)]
                let mut status = None;
                let mut wait_count = 0;
                loop {
                    if cancelled.load(Ordering::Relaxed) {
                        let _ = child.kill();
                        status = Some(Err(std::io::Error::new(
                            std::io::ErrorKind::Interrupted,
                            "Cancelled by user",
                        )));
                        break;
                    }
                    
                    match child.try_wait() {
                        Ok(Some(exit_status)) => {
                            status = Some(Ok(exit_status));
                            break;
                        }
                        Ok(None) => {
                            wait_count += 1;
                            if wait_count > 1200 {
                                let _ = child.kill();
                                status = Some(Err(std::io::Error::new(
                                    std::io::ErrorKind::TimedOut,
                                    "Removal timed out after 2 minutes",
                                )));
                                break;
                            }
                            thread::sleep(Duration::from_millis(100));
                        }
                        Err(e) => {
                            status = Some(Err(e));
                            break;
                        }
                    }
                }
                
                let _ = stdout_thread.join();
                let _ = stderr_thread.join();
                
                let status = status.unwrap();

                match status {
                    Ok(exit_status) if exit_status.success() => {
                        if let Ok(mut tracked) = tracker_packages.lock() {
                            if let Some(p) = tracked.get_mut(index) {
                                p.progress = 70;
                                p.message = "Cleaning dependencies...".to_string();
                            }
                        }

                        let mut auto_cmd = Command::new(PROGRAM);
                        auto_cmd
                            .arg("autoremove")
                            .stdin(Stdio::null())
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped());

                        if let Ok(mut auto_child) = auto_cmd.spawn() {
                            let mut auto_wait_count = 0;
                            loop {
                                match auto_child.try_wait() {
                                    Ok(Some(_)) => break,
                                    Ok(None) => {
                                        auto_wait_count += 1;
                                        if auto_wait_count > 600 {
                                            let _ = auto_child.kill();
                                            break;
                                        }
                                        thread::sleep(Duration::from_millis(100));
                                    }
                                    Err(_) => break,
                                }
                            }
                        }

                        if let Ok(mut tracked) = tracker_packages.lock() {
                            if let Some(p) = tracked.get_mut(index) {
                                p.state = ProgressState::Completed;
                                p.progress = 100;
                                p.message = "Removed!".to_string();
                            }
                        }
                    }
                    Ok(_) => {
                        if let Ok(mut tracked) = tracker_packages.lock() {
                            if let Some(p) = tracked.get_mut(index) {
                                p.state = ProgressState::Failed;
                                p.progress = 0;
                                p.message = "Removal failed".to_string();
                            }
                        }
                    }
                    Err(e) => {
                        if let Ok(mut tracked) = tracker_packages.lock() {
                            if let Some(p) = tracked.get_mut(index) {
                                p.state = ProgressState::Failed;
                                p.progress = 0;
                                p.message = format!("Error: {}", e);
                            }
                        }
                    }
                }

                thread::sleep(Duration::from_millis(100));
            })
        }).collect()
    };

    let cancelled_clone = Arc::clone(&cancelled);
    let removal_completed = tracker.run_with_updates(|| {
        remove_threads.iter().all(|t| t.is_finished())
    });
    
    if removal_completed.is_err() || !removal_completed.unwrap_or(true) {
        cancelled_clone.store(true, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(200));
    }

    for thread in remove_threads {
        let _ = thread.join();
    }
    
    let guard = tracker_packages_for_result.lock();
    if let Ok(packages) = guard {
        packages.iter().map(|p| PackageResult {
            name: p.name.clone(),
            status: p.state_label().to_string(),
        }).collect()
    } else {
        vec![]
    }
}
