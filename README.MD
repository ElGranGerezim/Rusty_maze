# Overview

A program built in rust that find a path from the start to the end of a maze. Writen to help me start understanding the basics of Rust.

Initially planning to use Graphs to represent the maze, I quickly learned how monumental of a task using graphs in rust is, so I quickly shifted to a custom struct using a 2 dimensional array to represent the maze instead. 

[Software Demo Video](https://youtu.be/snxGuteh5-c)

# Development Environment

Developed in Rust using Visual Studio Code and Rustup/Cargo.

Dependent on the [Serde](https://crates.io/crates/serde) and [Serde_Json](https://crates.io/crates/serde_json) crates for JSON I/O

# Useful Websites

* [The Rust Book](https://doc.rust-lang.org/book/index.html)
* Why graphs in rust are a bad idea for beginners
  * [Nick Cameron, Graphs and Arena Allocation](https://github.com/nrc/r4cppp/blob/master/graphs/README.md)
  * [Niko Matsakis, Modeling Graphs in Rust Using Vector Indices](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)
* [Serde Documentat](https://docs.rs/serde/latest/serde/)

# Future Work

* Learn graphics so I can make an actual window for this instead of just being a CL program
  * Possibly a GIF library so I can make an animation of the search
* Impliment different search algorithms to show the differences between them
* Write a helper program for generating mazes, hard-coding them by hand into the JSON is tedious