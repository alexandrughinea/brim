# BRIM
#### (home)Brew Remote Install Manager

<a href="https://www.buymeacoffee.com/alexandrughinea" title="BRIM (Brew Remote Install Manager)">
  <img src=".fixtures/logo.svg" alt="BRIM (Brew Remote Install Manager)" width="256px">
</a>


## What is BRIM?
#### The name stands for (home)Brew Remote Install Manager, we will refer to it as BRIM from now on.

1. BRIM is a very simple, yet effective Command-Line Interface (CLI) tool built in Rust.
2. It is designed to simplify the installation of multiple Homebrew packages described by remote resource (JSON in pre-alpha), in one shot. 
3. With BRIM, you can easily manage and install Homebrew packages from a centralized location, streamlining your entire setup process.
4. An excuse for myself to further learn Rust by maintaining it.
5. It is very lite and runs where Homebrew runs.
6. It is designed with security in mind.

## What is not BRIM?

1. BRIM is not associated in any way with the Homebrew project.
2. It was not built for any financial gains, and it will remain that way.

## Arguments

### `u` (short for URL)

example: `brim -u=https://mydomain.io/package-list.json`

The `url` argument takes a remote JSON URL that contains all the required package information.
The JSON structure expected in the response is an array of objects having the following fields:

- `name` (required, string): The name of the package.
- `cask` (optional boolean): Specify if the package is a cask (when applicable).
- `category` (optional, string): The category of the package.
- `url` (optional, string): URL to the formulae.


Example installation generated menu UI:

```
BRIM found 70 packages to install with Homebrew: [Page 1/6]
> [ ] vim - [installed]
[ ] git - [installed]
[ ] tmux - [installed]
[ ] htop - [installed]
[x] python3
[x] trash
[ ] tree - [installed]
[ ] jq - [installed]
[x] rlwrap
[ ] coreutils - [installed]
[x] neovim
[x] webstorm - [cask installed]
```

### `list`

The `list` argument prints out all the installed Homebrew packages on your system.

### `remove`

The `remove` argument lists every installed Homebrew package from your system eligible for removal.
You have the flexibility to select what you want to nuke (including its dependencies) out of your system.

## Features
1. Automatic dynamic menu generator based on the list you provide.
2. Installation menu for your dependencies.
3. Uninstall menu with automatic dependencies cleanup.
4. Support for casks.

If you'd like to make a feature suggestion please do so on the [issues](https://github.com/alexandrughinea/brim/issues)
page and let's discuss proposals there.

## Donations

If you like `BRIM`, thanks for considering supporting its development!
Your small support ensures that BRIM remains a reliable and efficient tool for managing your Homebrew installations.

If you'd like to make a donation, your generosity is greatly appreciated.

<a href="https://www.buymeacoffee.com/alexandrughinea" title="Buy me a beer">
  <img src=".fixtures/bmc_qr.png" alt="Donate" width="128px">
</a>

