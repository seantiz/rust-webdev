# Rust-JS Interoperability Journey

This is a record of my journey from Javascript web development to Rust! You'll find an overview of the thought process behind writing programs in Rust, and a guide to all the current, maintained platforms, libraries and techniques in the world of Rust-JS Interoperability.

## Overview

## Table of Contents

<!-- MarkdownTOC depth=4 -->

- [Differences Between Javascript and Rust](#js-vs-rust)
    - [Static Types](#static-types)
    - [Compile Times](#compile-times)
    - [Error Handling](#error-handling)
    - [Rust Concurrency and Javascript Async](#concurrency)
    - [No Garbage Collector](#no-gc)
    - [Ownership, Borrowing and Lifetimes](#ownership)
- [Two Ways to Make Ourselves at Home in Rust](#get-familiar)
    - [1. Functional Rust and Functional JS](#functional-coding)
    - [2. Rust Developer Map](#dev-map)
    - [Further Reading](#further-reading)

<!-- /MarkdownTOC -->

# Differences Between Javascript and Rust {#js-vs-rust}

## Static Types

## Compile Times

## Error Handling

## Rust Concurrency Compared to Javascript Async {#concurrency}

## No Garbage Collector {#no-gc}

The approach in memory management is the main difference to get to grips with in Rust. Unlike Javascript, there's no garbage collector. Keep in mind that Rust is a compiled language (like Typescript), and enforces ownership and borrowing rules at compile time. These rules are effectively Rust's memory-safe guarantee, and most of your initial time is spent working within that guarantee, so that you can later spot (for yourself, your domain and your end-user) when and where to work around the guarantee without breaking it.

## Ownership, Borrowing and Lifetimes {#ownership}

# Two Ways to Make Ourselves at Home in Rust {#get-familiar}

## 1. Practice Functional Rust and Functional JS {#functional-coding}

## 2. Grind Through a Rust Developer Map {#dev-map}

## Two Good Books for Further Reading {#further-reading}

