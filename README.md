# Rust-Typescript Interop Guide

This is a record of my journey from Typescript and Javascript web development to Rust!

You'll find an overview of the thought process behind writing programs in Rust when you're coming
from web development in Typescript, and a guide to all the current, maintained platforms, libraries
and techniques in the world of Rust-JS/TS Interoperability.

![Typescript to Rust Interop Development](/static/RustTSlight.drawio.svg)


## Overview

<!-- MarkdownTOC depth=4 -->

-   [Differences Between Typescript and Rust](#differences-between-typescript-and-rust)
    -   [Static Types](#static-types)
    -   [Compile Times](#compile-times)
    -   [Error Handling](#error-handling)
    -   [Rust Concurrency Compared to Javascript Async](#rust-concurrency-compared-to-javascript-async)
    -   [No Garbage Collector](#no-garbage-collector)
    -   [Ownership, Borrowing and Lifetimes](#ownership-borrowing-and-lifetimes)
-   [Making Ourselves at Home in Rust](#making-ourselves-at-home-in-rust)
    -   [Practice Functional Coding](#practice-functional-coding)
    -   [Grind Through a Rust Developer Map](#grind-through-a-rust-developer-map)
    -   [Two Good Books for Further Reading](#two-good-books-for-further-reading)
<!-- /MarkdownTOC -->


## Differences Between Typescript and Rust

Some differences below come with less mental overhead attached if you're familiar with writing
Typescript (a static language like Rust). If you're used to writing straight Javascript with no TS
layer on top, then you've got the extra task of getting used to a compiled language with (sometimes)
explicit data types.


### Primitives and Basic Types

You can [click here](/PRIMITIVES-BASIC-TYPES.md) for a full list of the basic types between
Javascript, Typescript and Rust.


### Static Typing

You can [click here](/STATIC-TYPING.md) to read this section in full.

### Set Operations

Typescript's operators for putting set theory into practice are straightforward: The `|` operator is
for set unions, and the `&` operator is for set intersections.

Rust's support for these two set operations is more nuanced:

-   Use Rust's enums for union operations and be explicit about which variant of the enum is in use
    at all times, to enforce safety and memory allocation/deallocation.
-   There's really no direct equivalent for intersection operations in Rust. You can achieve similar
    outcomes by using Rust traits, but the Rust approach is more about defining and implementing
    behavior rather than directly combining types.

### Compile Times

### Error Handling

### Rust Concurrency Compared to Javascript Async

### No Garbage Collector

The approach in memory management is the main difference to get to grips with in Rust. Unlike
Javascript, there's no garbage collector.

Keep in mind that Rust is a compiled language (like Typescript), and enforces **ownership and
borrowing rules** at compile time. These rules are Rust's memory-safe guarantee; most of your
journey starts by working within that guarantee, so that you can later spot (for yourself, your
domain and your end-user) when and where to work around the guarantee without breaking it.

We'll touch upon ownership and borrowing briefly in the next section, but I'd argue it's more
important to get comfortable with Rust inside your IDE early on in your journey. Later on, we can
get deep into concepts and the shift in mindset to Rust memory management.

### Ownership, Borrowing and Lifetimes

## Making Ourselves at Home in Rust

### 1. Practice Functional Rust and Functional TS

### 2. Grind Through a Rust Developer Map

### Two Good Books for Further Reading
