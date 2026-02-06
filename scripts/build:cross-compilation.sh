targets=()
while IFS= read -r line; do
    if [[ "$line" =~ ^\[target\.([^\]]+)\.dependencies\]$ ]]; then
        target="${BASH_REMATCH[1]}"
        targets+=("$target")
    fi
done < Cargo.toml

for target in "${targets[@]}"; do
    echo "Building for target: $target"
    cargo build --target="$target" --release
done
