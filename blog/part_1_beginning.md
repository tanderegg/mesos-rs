# Reimplementing Mesos in Rust

## Why?

This project came about since I had an interest in understanding both Rust and Mesos better.  I use Mesos on a regular basis, have dabbled in contributing, and would like to get more involved in framework writing and code contribution.  At the same time, I've come to greatly enjoy the Rust programming language, so I thought that reimplementing Mesos in Rust would both force me to understand Mesos better, by going through most of the source code and understanding how it works at a low level, while also giving me a chance to learn more about a new language.

In addition to this, Rust and Mesos are a good natural fit, as it was original considered as the language that would be used to write Mesos in the first place.  At the time, it was not yet in a mature enough state, but I believe it is now.  The Mesos team took the C++ route, which was the next best option at the time, and ended up implementing the `Stout` library, which in essence is a collection of low-level primitives and utility functionality that brought C++ up to speed with where Rust aspires to be.

So, one immediate advantage is that the `Stout` code base can probably be set aside for this project, as most of the features are already in Rust as part of the core language or standard library.  We'll see as I go how true that statement is!

## Goals

## Getting Started

### Mesos Architecture Overview

### Project setup

### Libprocess

Ran into this error: https://github.com/rust-lang/rust/issues/45838
