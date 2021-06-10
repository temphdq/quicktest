<h1 align="center">⛑ Quick Test CLI</h1>

<p align="center">Command Line Interface (CLI) for stress testing in competitive programming</p>

[![Current Crates.io Version](https://img.shields.io/crates/v/quicktest.svg)](https://crates.io/crates/quicktest) [![Quick Test](https://circleci.com/gh/LuisMBaezCo/quicktest.svg?style=shield)](https://app.circleci.com/pipelines/github/LuisMBaezCo/quicktest) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT) [![](https://img.shields.io/crates/d/quicktest)](https://crates.io/crates/quicktest)

### Installation

If you already have Rust on your system:

```sh
cargo install quicktest
```

If you don't have rust installed on your system, the following command will install Rust and the CLI at once:

Shell (Linux, Mac):
```sh
curl https://sh.rustup.rs -sSf | sh  && cargo install quicktest
```

### Usage

```sh
quicktest --help
```

```
quicktest
CLI for stress testing in competitive programming contest

USAGE:
    quicktest <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cmp     Check the correctness of the <target-file> comparing it with <slow-file>
            with input test generated by <gen-file>
    help    Prints this message or the help of the given subcommand(s)
    tle     Check TLE
```

### TLE Subcommand

```sh
quicktest tle --help
```

```shell
quicktest-tle
Check TLE

USAGE:
    quicktest tle [FLAGS] [OPTIONS] --gen-file <gen-file> --target-file <target-file>

FLAGS:
    -h, --help          Prints help information
    -s, --save-cases    Save test cases
    -b, --tle-break     Break if Time Limit Exceeded (TLE) occurs
    -V, --version       Prints version information

OPTIONS:
    -g, --gen-file <gen-file>          Generator File
    -t, --target-file <target-file>    Target File
    -n, --test-cases <test-cases>      Number of test cases [default: 1000]
    -o, --timeout <timeout>            Timeout TLE [default: 2000]
```

### Cmp Subcommand

```sh
quicktest cmp --help
```

```shell
quicktest-cmp
Check the correctness of the <target-file> comparing it with <slow-file> with input test generated by <gen-file>

USAGE:
    quicktest cmp [FLAGS] [OPTIONS] --correct-file <correct-file> --gen-file <gen-file> --target-file <target-file>

FLAGS:
    -h, --help          Prints help information
    -s, --save-cases    Save test cases
    -V, --version       Prints version information
    -b, --wa-break      Break if Wrong Answer (WA) occurs

OPTIONS:
    -c, --correct-file <correct-file>    Correct File
    -g, --gen-file <gen-file>            Generator File
    -t, --target-file <target-file>      Target File
    -n, --test-cases <test-cases>        Number of test cases [default: 1000]
    -o, --timeout <timeout>              Timeout TLE [default: 2000]
```

## Supported Languages

| Language           |       Version          |
|--------------------|------------------------|
| C++                | -std=c++17             |
| Python             | Version 3              |


# License
Licensed under either of these:
* MIT license ([LICENSE-MIT](https://github.com/LuisMBaezCo/quicktest/blob/main/LICENSE) or https://opensource.org/licenses/MIT)