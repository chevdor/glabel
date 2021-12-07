VERSION := `toml get cli/Cargo.toml package.version | jq -r`
TARGET_DIR := "target/release"
export TAG:=`toml get cli/Cargo.toml "package.version" | jq -r .`

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

# Test / watch
test:
	cargo watch -x "test -- --no-capture"

# Test including ignored tests
test_all:
	cargo test -- --include-ignored

# Generate usage samples
_usage:
    cargo run -q -- --help > doc/help.adoc
    cargo run -q -- get --help > doc/get.adoc
    cargo run -q -- get chevdor/glabel > doc/sample_terminal.txt
    cargo run -q -- get chevdor/glabel -o doc/sample_yaml.yml

# Generate documentation
doc: _usage
	cargo doc -p glabel -p glabellib --all-features --no-deps

# Run rustfmt
_fmt:
	cargo +nightly fmt --all

# Run clippy
_clippy:
	cargo +nightly clippy --all-features --all-targets

_deny:
	cargo deny check

# Run checks such as clippy, rustfmt, etc...
check: _clippy _fmt _deny

# Minor bump, can be used once the release is ready
bump:
	cargo workspaces version --no-git-commit


# Generate the readme as .md
md:
    #!/usr/bin/env bash
    asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md

release: check test_all bump doc md

coverage:
	#!/usr/bin/env bash
	export RUSTFLAGS="-Zinstrument-coverage"
	export LLVM_PROFILE_FILE="chevdor-%p-%m.profraw"
	cargo +nightly build
	cargo +nightly test
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
	open target/debug/coverage/index.html
	find . -type f -name '*.profraw' -exec rm '{}' \;

tag:
    #!/bin/sh
    echo Tagging version v$TAG
    git tag "v$TAG" -f
    git tag | sort -Vr | head
