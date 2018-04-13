Rust Security Talk
==================

This repository contains all the sample code for the talk 'Secure Corrdoing with Rust' (BSides Canberra, 2018).

Slides for the talk are available [here](http://securecode.rs/bsidescbr2018.pdf)


Getting Started
---------------

All the examples in this repo can be built with a pretty standard C and Rust development environment.
However if you don't have that setup you can easily build the supplied docker container and have all the necessary tools in one place.

To build the docker container for this repo:
`docker build -f Dockerfile -t rust-security-talk:latest .`

If you're not already familiar with [Rust](https://www.rust-lang.org/en-US/) I highly recommend reading [The Rust Programming Language 2ed](https://doc.rust-lang.org/book/second-edition/). Apart from being a good guide to unsafe development with Rust [The Rustonomicon](https://doc.rust-lang.org/nomicon/) also provides insights into the security aspects of the language.


Project Layout
--------------

Each topic in the talk has it's own directory in this respository. Any slide from the presentation with code references the specific directory where that code can be found. e.g. slide 19: dangling_pointer/2/rust
