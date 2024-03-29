VERSION := `toml get cli/Cargo.toml package.version | jq -r`
TARGET_DIR := "target/release"
export TAG:=`toml get cli/Cargo.toml "package.version" | jq -r .`

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

# Test / watch
test:
	cargo watch -x "test -- --nocapture"

# Test including ignored tests
test_all:
	cargo nextest run -- --include-ignored

# Generate usage samples
_usage:
	#!/usr/bin/env bash
	cargo run -q -- --help > doc/help.adoc
	cargo run -q -- get --help > doc/get.adoc
	SAVE=$TOKEN
	export TOKEN="<your admin token>"
	cargo run -q -- apply --help > doc/apply.adoc
	cargo run -q --features=wipe -- wipe --help > doc/wipe.adoc
	export TOKEN=$SAVE
	cargo run -q -- get chevdor/glabel > doc/sample_terminal.txt
	cargo run -q -- get chevdor/glabel -o doc/sample_yaml.yaml
	tera --template templates/doc.md.tera doc/sample_yaml.yaml > doc/sample_doc.md

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
check: _clippy _fmt

# Minor bump, can be used once the release is ready
bump:
	cargo workspaces version --no-git-commit

# Generate the readme as .md
md:
	#!/usr/bin/env bash
	asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md
	cp README.md cli/

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
    #!/usr/bin/env bash
    echo Tagging version v$TAG
    git tag "v$TAG" -f
    git tag | sort -Vr | head

tera:
    #!/usr/bin/env bash
    tera --template templates/doc.md.tera doc/sample_yaml.yaml

build:
	cargo  build --profile release --locked --features "wipe"
	mv ./target/release/glabel ./target/release/glabel-wipe
	cargo  build --profile release --locked --features ""

sha256:
	#!/usr/bin/env bash
	cd target/release
	for file in $( find . -d 1 -type f -exec test -x {} \; -print ); do
		echo Creating sha256 for $file
		shasum -a 256 $file > $file.sha256
		shasum -c $file.sha256
	done

sign:
	#!/usr/bin/env bash
	for file in $( find ./target/release -d 1 -type f -exec test -x {} \; -print ); do
		echo Signing binaries, get your Yubikey ready
		gpg --sign --default-key $KEY --armor --output $file.asc --detach-sig $file
		gpg --verify $file.asc
	done

build_sha256_and_sign: build sha256 sign
