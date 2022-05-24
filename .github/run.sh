#!/bin/bash

set -e

test=${fix:-y}
lint=${fix:-y}
fix=${fix:-y}
dir=${dir:-}

# Read the arguments
while [ $# -gt 0 ]; do
   if [[ $1 == *"--"* ]]; then
        param="${1/--/}"
        declare $param="$2"
   fi

  shift
done

# Find manifests
if [ -z "$dir" ]; then
	manifests=(**/Cargo.toml)
else
	manifests=("$dir/Cargo.toml")
fi

green='\033[1;32m'
no_color='\033[0m'
for m in "${manifests[@]}"; do
	name="$(dirname $(readlink -f $m))"
	name="$(basename $name)"

	printf "Project dir: ${green}$name${no_color}\n"

	if [ "$test" == "y" ]; then
		cargo test --all-features --manifest-path $m -- --ignored
	fi

	if [ "$lint" == "y" ]; then
		if [ "$fix" == "y" ]; then
			cargo fmt --manifest-path "$m" -- -l
			cargo clippy --manifest-path "$m" --fix --allow-dirty --allow-staged --no-deps
		else
			cargo fmt --manifest-path "$m" -- --check
			cargo clippy --manifest-path "$m" -- -D warnings --no-deps
		fi
	fi
done