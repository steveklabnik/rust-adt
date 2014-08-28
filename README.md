# rust-adt [![Build Status](https://travis-ci.org/Indiv0/rust-adt.svg?branch=master)](https://travis-ci.org/Indiv0/rust-adt)

A collection of [abstract data types](http://en.wikipedia.org/wiki/Abstract_data_type), written in Rust.

## Purpose

This project is intented to help me learn and experiment with Rust concepts.

It is also meant to help me keep track of breaking Rust changes, and how to fix them.

## ADTs
### Implemented

* Stack
    * Vec Stack

### TBD

* Stack
    * Linked List Stack
* Queue
* List

### Won't Implement

I won't be implementing Linked Lists here.
Singly linked lists in Rust are [difficult to do](http://people.mozilla.org/~lbergstrom/Korea2013/RustPatterns.pdf) and provide almost no memory/speed advantage over doubly linked lists.
Doubly linked lists are difficult to implement, but a very concise and powerful implementation is available in the rust standard library as [DList](https://github.com/rust-lang/rust/blob/master/src/libcollections/dlist.rs).
