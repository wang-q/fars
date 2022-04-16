# fars


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
