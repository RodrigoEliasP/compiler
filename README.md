# Compiler

This is a simple compiler (the extremely elaborate project name may have told you that) project inspired by [_Crafting Interpreters_](https://craftinginterpreters.com) by Robert Nystrom. By the time this readme was last updated, the author of this repository was in the Scanning chapter of the book. It includes an implementation of a language interpreter for a small language called Lox, with both a tree-walk interpreter and a bytecode virtual machine.

## Table of Contents

- [Introduction](#introduction)
- [Running](#running)
- [Resources](#resources)
- [License](#license)

## Introduction

For the sake of learning new things, I've decided to implement the java version of this interpreter in rust, with automated test, although at the expense of investing more time and brain juice trying to adapt java to rust.

## Running

To run the project you're gonna need to install [rust](https://www.rust-lang.org/tools/install) if you don't have it yet.
To get started, checkout the project folder and execute

```
cargo run
```

ro run the REPL or

```
cargo run script.lox
```

ro run a script file

## Resources

[Crafting Interpreters](https://craftinginterpreters.com): The book on which this project is based

## License

It's MIT, check out the LICENSE file for more details.
