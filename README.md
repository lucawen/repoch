# REPOCH

[![Build Status](https://travis-ci.com/lucawen/repoch.svg?branch=master)](https://travis-ci.com/lucawen/repoch)
[![Crates.io](https://img.shields.io/crates/v/repoch.svg)](https://crates.io/crates/repoch)


REPOCH is cli to convert epoch timestamp datetime and datetime to timestamp.

<p align="center">
  <img width="100%" src="https://cdn.jsdelivr.net/gh/lucawen/repoch/example.gif">
</p>


Its convert a epoch time from utc to datetime.

#### Building for source
For build the binary just:
```sh
$ cargo build
```
To run, just run this example:
```sh
$ cargo run -- 1558150671
```
### Installation
Install simple typing:s

```sh
cargo install repoch
```

### Documentation
The documentation, for now, is the help return of tool:

```sh
Convert the epoch value to datetime and datetime to epoch (everyting as utc)

USAGE:
    repoch [FLAGS] [OPTIONS] <value>

FLAGS:
    -h, --help       Prints help information
    -d, --date       If will convert from date to epoch or not
    -V, --version    Prints version information

OPTIONS:
    -f, --format <format>    Conversion format, Epoch is output and Date is input [default: %Y-%m-%d %H:%M:%S]

ARGS:
    <value>    Value to convert
```


License
----

MIT


**Free Software, Hell Yeah!**
