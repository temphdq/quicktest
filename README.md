<p align="center">
  <img width="128" height="128" src="assets/logo/quicktest-512x512.png">
</p>

<h1 align="center">Quick Test CLI</h1>

<p align="center">Command Line Interface (CLI) for Stress Testing for Competitive Programming</p>

<h2 align="center">📖 <a href="https://luchobazz.github.io/quicktest/docs/intro">Docs</a></h2>

[![Current Crates.io Version](https://img.shields.io/crates/v/quicktest.svg)](https://crates.io/crates/quicktest) [![Quick Test](https://circleci.com/gh/LuchoBazz/quicktest.svg?style=shield)](https://app.circleci.com/pipelines/github/LuchoBazz/quicktest) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue.svg)](https://github.com/LuchoBazz/quicktest)

## Demo
![cmp gif](./website/static/gif/main.gif)

## Documentation

**Table of Contents**

- [Getting Started](#getting-started)
  - [Installation](#installation)
- [Introduction](#introduction)
- [Disclaimer](#disclaimer)
- [Commands](#commands)
- [How to Contribute](#how-to-contribute)
- [Bug reports](#bug-reports)
- [Supported Languages](#supported-languages)
- [Compilation and Execution Commands](#compilation-and-execution-commands)
- [License](#license)
- [Credits](#credits)

## Getting Started

### Installation

Installation tutorial for Linux, Windows and macOS can be found [here](https://luchobazz.github.io/quicktest/docs/getting-started/installation) on the docs website

### Disclaimer
QuickTest, supports Windows and Linux. The CLI has been tested successfully on these platforms, but functionality on macOS has not yet been verified. Users on macOS are advised to proceed with caution and provide feedback on any issues encountered.

## Introduction

Quick Test CLI is a project to perform stress testing in competitive programming contests in an easy and fast way, focusing only on the contest.

Currently, Quick Test CLI supports three types of tests which are listed below:

| [quicktest cmp](https://luchobazz.github.io/quicktest/docs/usage/cmp)                               | [quicktest stress](https://luchobazz.github.io/quicktest/docs/usage/stress)                               | [quicktest check](https://luchobazz.github.io/quicktest/docs/usage/check)                              |
|------------------------------------------------|------------------------------------------------|----------------------------------------------|
| ![cmp gif](./website/static/gif/cmp.gif) | ![stress gif](./website/static/gif/stress.gif) | ![check gif](./website/static/gif/check.gif) |

* `quicktest cmp | qt cmp`: It checks the correctness of the algorithm we want to verify by comparing it with a brute force solution which is usually very slow, but is 100% sure to provide the correct solution.
    * **Sample:**

        ```shell
        quicktest cmp --target-file=main.cpp --correct-file=correct.cpp --gen-file=gen.cpp
        # Or shorter:
        qt cmp -t main.cpp -c correct.cpp -g gen.cpp --tout 1000 --tc 1000
        ```

* `quicktest stress | qt stress`: Verify that the code execution time does not exceed what is allowed, using a random generator for multiple test cases.

    **Note:** In this scenario there is no slower solution with which to compare the correction.

    * **Sample:**
        ```shell
        quicktest stress --target-file=main.cpp --gen-file=gen.cpp
        # Or shorter:
        qt stress -t main.cpp -g gen.cpp --tout 1000 --tc 1000
        ```

*  `quicktest check | qt check`: In some problems more than one answer is accepted, so the `quicktest cmp` command would not work correctly, in this case a script checker is used to verify the correctness of the algorithm.

    * **Sample:**
        ```shell
        quicktest check --target-file=main.cpp --checker-file=correct.cpp --gen-file=gen.cpp
        # Or shorter:
        qt check -t main.cpp -c check.cpp -g gen.cpp --tout 1000 --tc 1000
        ```

* `quicktest output | qt output`: run all test cases that match a prefix and save the response to an output file
    * **Sample:**
        ```shell
        quicktest output --target-file=main.cpp --prefix=testcase_ac
        # Or shorter:
        qt output -t main.cpp -p test_cases/testcase_ac --tout 1000
        ```

## Commands

**Note:** you can use the long command `quicktest` or the alias `qt`

* `quicktest cmp | qt cmp`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-c=<value> | --correct-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --tc=<value> [default: 1000]`
    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--prefix=<value> | -p=<value>` conflict with `--gen-file` (Only one can be used at a time)
    * `--diff`  Show differences between the expected file and the output file

---

* `quicktest stress | qt stress`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --tc=<value> [default: 1000]`
    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--prefix=<value> | -p=<value>` conflict with `--gen-file` (Only one can be used at a time)

---

* `quicktest check | qt check`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-c=<value> | --checker-file=<value>`
    * `-g=<value> | --gen-file=<value>`

    **Other Options**

    * `--test-cases=<value> | --tc=<value> [default: 1000]`
    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--prefix=<value> | -p=<value>` conflict with `--gen-file` (Only one can be used at a time)

---

* **Flags of the `cmp`, `stress` and `check` subcommands**

    * `--break-bad | --break`  Break if WA, TLE or RTE states occurs
    * `--run-ac`     Run test cases Accepted
    * `--run-all`    Run all test cases
    * `--run-rte`    Run test cases Run Time Error
    * `--run-tle`    Run test cases Time Limited Exceeded
    * `--run-wa`     Run test cases Wrong Answer
    * `--save-all`   Save all test cases
    * `--save-bad`   Save only bad cases with WA, TLE or RTE states

---
* `quicktest output | qt output`
    
    **Required Options**

    * `-t=<value> | --target-file=<value>`
    * `-p=<value> | --prefix=<value>`

    **Other Options**

    * `--timeout=<value> | --tout=<value> [default: 2000]` Unit of time: `ms`
    * `--memory-limit=<value> | --ml=<value> [default: 1000000000 - 1GB]` Unit of time: `bytes`
    * `--break-bad | --break`  Break if WA, TLE or RTE states occurs
    * `--save-out`   Save the output of the target file for each test case

---
* `quicktest setup | qt setup`

    **Subcommand**

    * `config` Subcommand that allows to change C++ settings

        **Options**

        * `--label=<value>` Label with the path of the configuration that you want to change
        * `--value=<value>` value you want to change a selected label to

---
* `quicktest example | qt example`
    
    **Flags**
    
    * `--check`   Show examples of the check subcommand
    * `--cmp`     Show examples of the cmp subcommand
    * `--stress`  Show examples of the stress subcommand
    * `--output`     Show examples of the output subcommand
    * `--setup`   Show examples of the setup subcommand
    
    **Nota:** can only use one flag at a time

## How to Contribute

If you are interested in contributing to the Quick Test CLI project, please take a look at the [Contribute](./CONTRIBUTING.md) guide

### Bug reports

You can report any bugs [here](https://github.com/LuchoBazz/quicktest/issues).

## Supported Languages

| Language           |
|--------------------|
| C++                |
| Java               |
| Python             |
| Rust Lang          |
| Go Lang            |
| GNU C              |
| Kotlin             |

## Compilation and Execution Commands

| Language  | Compile / Interpreter                                                                                                                                                         | Execution Command        |
|-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------|
| C++17     | `g++ -std=c++17 -Wall -DONLINE_JUDGE=1 -o .qt/main main.cpp`                                                                                                                  | `./.qt/main`             |
| Java      | `javac -d .qt/ Main.java`                                                                                                                                                     | `java -cp .qt/ Main`     |
| Python3   |                                                                                                                                                                               | `python3 main.py`        |
| Rust Lang | `cp main.rs ~/.quicktest/rust/src/main.rs && cargo build --release --quiet --manifest-path ~/.quicktest/rust/Cargo.toml && cp ~/.quicktest/rust/target/release/rust .qt/main` | `./.qt/main`             |
| Go Lang   | `cp main.go ~/.quicktest/go_mod/main.go && go build -buildmode=exe -o ./.qt/main ~/.quicktest/go_mod/main.go`                                                                 | `./.qt/main`             |
| GNU C     | `gcc -std=gnu11  -lm main.c -o .qt/main`                                                                                                                                      | `./.qt/main`             |
| Kotlin    | `kotlinc main.kt -include-runtime -d .qt/main.jar`                                                                                                                            | `java -jar .qt/main.jar` |


## License
Licensed under either of these:
* MIT license ([LICENSE-MIT](https://github.com/LuchoBazz/quicktest/blob/main/LICENSE) or https://opensource.org/licenses/MIT)

---

## Credits

> <sub>Logo image based on the one made by <a href="https://www.flaticon.com/authors/freepik" id-flaticon="owl_2369306;hexagonal_73861" title="Freepik">Freepik</a> for <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a>

> <sub>Documentation was based on <a href="https://searleser97.github.io/cpbooster/" title="CP Booster">cpbooster</a>

> <sub>Installation scripts were based on <a href="https://github.com/denoland/deno_install" title="Flaticon">deno_install</a>