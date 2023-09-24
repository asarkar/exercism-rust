#!/bin/bash

set -e

export RUST_BACKTRACE=1
export RUSTFLAGS="-D warnings"

no_test=0
no_lint=0
no_fix=0

while (( $# > 0 )); do
   case "$1" in
   	--help)
			printf "run.sh [OPTION]... [DIR]\n"
			printf "options:\n"
			printf "\t--help			Show help\n"
			printf "\t--no-test		Skip tests\n"
			printf "\t--no-lint		Skip linting\n"
			printf "\t--no-fix		Do not apply linter suggestions\n"
			exit 0
      	;;
      --no-test)
			no_test=1
			shift
      	;;
      --no-lint)
			no_lint=1
			shift
			;;
		--no-fix)
			no_fix=1
			shift
			;;
		*)
			break
	      ;;
   esac
done

bin_dir=""
if [[ "$OSTYPE" == "darwin"* ]]; then
	bin_dir="$HOME/.cargo/bin/"
fi

manifests=()
if [[ -z "$1" ]]; then
	manifests=(**/Cargo.toml)
else
	manifests+=("$1/Cargo.toml")
fi

green='\033[1;32m'
no_color='\033[0m'
for m in "${manifests[@]}"; do
	name="$(dirname "$(readlink -f "$m")")"
	name="$(basename "$name")"

	printf "Project dir: %b%s%b\n" "$green" "$name" "$no_color"

	if (( no_test == 0 )); then
		"$bin_dir"cargo test --all-features --manifest-path "$m" -- --include-ignored --nocapture
	fi

	if (( no_lint == 0 )); then
		if (( no_fix == 0 )); then
			"$bin_dir"cargo fmt --manifest-path "$m" -- -l
			"$bin_dir"cargo clippy --manifest-path "$m" --fix --allow-dirty --allow-staged --no-deps
		else
			"$bin_dir"cargo fmt --manifest-path "$m" -- --check
		fi
		"$bin_dir"cargo clippy --manifest-path "$m" -- -D warnings --no-deps
	fi
done