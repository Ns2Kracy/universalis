_list:
    @just --list

alias t := test
alias br := build-release
alias p := publish

test:
    cargo nextest run

build-release:
    cargo build --release

test-publish:
    cargo publish --dry-run --registry crates-io

publish:
    cargo publish --registry crates-io