# Dogrep

Dogrep is a small version of the grep command line tool implemented in Rust to find specified content in a file.

## Features

* Case insensitive search is supported.
* Support reverse lookup.
* Support tag matching line numbers.
* Statistics on the number of matches is supported.

## Install

> **Note**: Before using dogrep, make sure you have the following software and libraries installed on your system:
* Rust compiler

1. Clone the project to the local repository.

```shell
git clone git@github.com:FitnessDoge/dogrep.git
```

2. Enter project directory.

```shell
cd dogrep
```

3. Compile project.

```shell
cargo build --release
```

## Usage

```shell
dogrep [Options] <Pattern> <Filename>
```
* Options
  * -i : Ignore case.
  * -v : Reverse search.
  * -n : The tag matches the line number.
  * -c : Count the number of matches.

* Pattern : The pattern you want to match.
* Filename :  The name of the file you want to look for.