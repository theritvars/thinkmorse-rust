# thinkmorse-rust [![Build Status](https://travis-ci.org/RichusX/thinkmorse-rust.svg?branch=master)](https://travis-ci.org/RichusX/thinkmorse-rust)
A Rust version of the Python thinkmorse script

## Prerequisites
To have the control over the lid LED you need `ec_sys` kernel module loaded with write support enabled. To do this, add `ec_sys.write_support=1` to your kernel parameters. 

## Usage
Simply run the binary as sudo and provide the text you wish to display in morse as an argument.
```
sudo ./thinkmorse <string>
```
