# `fars` and `faops`

```bash
brew install fars
brew install faops

brew install hyperfine
brew install neofetch

```

## System info

* Ryzen 7 5800

```text
wangq@R7
--------
OS: Ubuntu 20.04.4 LTS on Windows 10 x86_64
Kernel: 5.10.102.1-microsoft-standard-WSL2
Uptime: 6 hours, 41 mins
Packages: 1244 (dpkg), 344 (brew)
Shell: bash 5.0.17
Theme: Adwaita [GTK3]
Icons: Adwaita [GTK3]
Terminal: Windows Terminal
CPU: AMD Ryzen 7 5800 (16) @ 3.393GHz
GPU: 9cdb:00:00.0 Microsoft Corporation Device 008e
Memory: 401MiB / 7877MiB

```

## `fars size`

```bash
hyperfine --warmup 1 --export-markdown size.md.tmp \
    'cat tests/fars/ufasta.fa | fars  size stdin > /dev/null' \
    'cat tests/fars/ufasta.fa | faops size stdin > /dev/null' \
    'fars  size tests/fars/ufasta.fa.gz > /dev/null' \
    'faops size tests/fars/ufasta.fa.gz > /dev/null'

```

| Command                 | Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:------------------------|----------:|---------:|---------:|------------:|
| fars  size ufasta.fa    | 1.7 ± 0.2 |      1.3 |      3.2 | 1.06 ± 0.19 |
| faops size ufasta.fa    | 1.7 ± 0.2 |      1.3 |      3.0 | 1.04 ± 0.17 |
| fars  size ufasta.fa.gz | 1.8 ± 0.2 |      1.3 |      4.1 | 1.06 ± 0.19 |
| faops size ufasta.fa.gz | 1.7 ± 0.2 |      1.2 |      3.7 |        1.00 |

## `fars some`

```bash
hyperfine --warmup 1 --export-markdown some.md.tmp \
    'fars  some tests/fars/ufasta.fa.gz tests/fars/lst.txt > /dev/null' \
    'faops some tests/fars/ufasta.fa.gz tests/fars/lst.txt stdout > /dev/null' \
    'fars  some -i tests/fars/ufasta.fa.gz tests/fars/lst.txt > /dev/null' \
    'faops some -i tests/fars/ufasta.fa.gz tests/fars/lst.txt stdout > /dev/null'

```

| Command       | Mean [ms] | Min [ms] | Max [ms] |    Relative |
|:--------------|----------:|---------:|---------:|------------:|
| fars  some    | 2.5 ± 0.2 |      1.9 |      3.4 | 1.01 ± 0.11 |
| faops some    | 2.5 ± 0.2 |      1.9 |      4.0 |        1.00 |
| fars  some -i | 2.5 ± 0.2 |      2.0 |      3.6 | 1.02 ± 0.11 |
| faops some -i | 2.5 ± 0.2 |      1.9 |      3.9 | 1.01 ± 0.12 |
