_list:
    @just --list

alias t := test
alias br := build-release
alias p := publish

test:
    cargo nextest run

build-release:
    cargo build --release

publish:
    cargo publish --registry crates-io