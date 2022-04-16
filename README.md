# fars

[![Publish](https://github.com/wang-q/fars/actions/workflows/publish.yml/badge.svg)](https://github.com/wang-q/fars/actions)
[![Build](https://github.com/wang-q/fars/actions/workflows/build.yml/badge.svg)](https://github.com/wang-q/fars/actions)
[![Codecov branch](https://img.shields.io/codecov/c/github/wang-q/fars/master.svg)](https://codecov.io/github/wang-q/fars?branch=master)
[![Crates.io](https://img.shields.io/crates/v/fars.svg)](https://crates.io/crates/fars)
[![Lines of code](https://tokei.rs/b1/github/wang-q/fars?category=code)](https://github.com//wang-q/fars)

## Install

Current release: 0.0.1

```bash
cargo install fars

# or
brew install fars

```

## SYNOPSIS

* `fars`

```text

```

## EXAMPLES

```bash
diff \
    <(target/debug/fars size tests/fars/ufasta.fa) \
    <(faops size tests/fars/ufasta.fa)

target/debug/fars region tests/fars/ufasta.fa

target/debug/fars some tests/fars/ufasta.fa tests/fars/lst.txt

```
