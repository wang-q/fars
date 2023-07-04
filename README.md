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
$ fars
fars 0.0.1
wang-q <wang-q@outlook.com>
`fars` is a lightweight tool for operating sequences in the fasta format

USAGE:
    fars [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    region    Extract regions from a FA file
    size      Count total bases in FA file(s)
    some      Extract some FA records

```

## EXAMPLES

```bash
diff \
    <(fars size tests/fars/ufasta.fa) \
    <(faops size tests/fars/ufasta.fa)

fars region tests/fars/ufasta.fa

fars some tests/fars/ufasta.fa tests/fars/lst.txt

```
