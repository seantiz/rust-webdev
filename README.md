# Rust-Typescript Interop Guide

This is my personal recap on writing cross-platform between Rust, Typescript and Javascript.

In a nutshell, you'll find:

1. A recap on the main differences (from primitives + basic types to life without a garbage
   collector)
2. Two paths to get comfortable with Rust: You can practice common ground between your working
   knowledge of webdev languages and
   [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html), or you can take a
   more analytical approach to grinding your way through a Rust Developer Map.
3. Further Reading on Rust concepts so you can get your processes and threads to inform each other
   meaningfully.
4. Rust-JS-Interop tech radar

<br>
<br>
<br>

![Typescript to Rust Interop Development](/static/RustTSlight.drawio.svg)

# Overview

<!-- MarkdownTOC depth=4 -->

- [Differences Between TypeScript and Rust](#differences-between-typescript-and-rust)
    - [Syntax Differences](#syntax-differences)
        - [Primitives and Basic Types](#primitives-and-basic-types)
        - [More On Static Typing](#more-on-static-typing)
        - [Set Operations](#set-operations)
    - [Different Developer Experience](#different-developer-experience)
        - [So Why the Tradeoffs in (Initial) Developer Experience?](#so-why-the-tradeoffs-in-developer-experience)
        - [Some Add-ons for Better Rust DX](#some-add-ons-for-better-rust-dx)
        - [Cleaning Out Cargo Artefacts](#cleaning-out-cargo-artefacts) ⚠️
    - [Error Handling](#error-handling)
    - [No Garbage Collector](#no-garbage-collector)
    - [Rust's Ownership Model and Concurrency vs JavaScript's Async Approach](#rusts-ownership-model-and-concurrency-vs-javascripts-async-approach)
        - [Ownership and Borrowing](#ownership-and-borrowing)
        - [Threading](#threading)
        - [Async/Await Runtime Cost](#asyncawait-runtime-cost)
        - [Cancellation and Blocking](#cancellation-and-blocking)
- [Feeling at Home in Rust](#feeling-at-home-in-rust)
    - [Practice Functional Coding](#practice-functional-coding)
    - [Grind Through a Rust Developer Map](#grind-through-a-rust-developer-map)
    - [Two Good Books for Further Reading](#two-good-books-for-further-reading)
<!-- /MarkdownTOC -->

# Differences Between Typescript and Rust

## Syntax Differences

Some differences below come with less mental overhead attached if you're familiar with writing
Typescript (a static language like Rust). If you're used to writing straight Javascript with no TS
layer on top, then you've got the extra task of getting used to a compiled language with (sometimes)
explicit data types.

### Primitives and Basic Types

You can [click here](/PRIMITIVES-BASIC-TYPES.md) for a full list of the basic types between
Javascript, Typescript and Rust.

### More On Static Typing

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

## Different Developer Experience

Just to be clear: "Developer Experience" here refers to quality of life between you, your IDE and runtime enviroments. The fated "DX"!

There's no getting around some noticeable tradeoffs in Rust, especially when it comes to compile times through Cargo. It's also harder to come by hot reloading between saves in a Rust dev environment (but a [debugging solution on this later](#some-add-ons-for-better-rust-dx)).

Cargo is Rust's package manager and build system. Aside from using the `rustc` command on single
`.rs` scripts, Cargo is the way to build your Rust repository into final OS-native executables.

If you're on MacOS or Linux and yet to install Cargo, the simplest way is executing the `rustup`
script from your terminal:

```Installing Cargo
curl https://sh.rustup.rs -sSf | sh
```

The bigger your project gets and the more library dependencies you bring into your `Cargo.toml`, the
more you'll notice the compilation times go up.

### So Why the Tradeoffs in Developer Experience?

There are valid design decisions behind the bigger RS compile times:

-   Rust performs extensive compile-time optimisation on code to get your final executables running
    as fast (and safe) as possible on the operating system. Rust also supports cross-compilation for
    true cross-platform development and even across different architectures. Typescript's build
    tools (like Rollup, Webpack, etc.) do not deal with the same concerns, given they're compiling
    to run inside a Javascript engine.

-   Rust performs extensive compile-time checking to enforce memory management. The first two-dozen
    times you try to compile in Rust as a beginner, it feels like an editor drawing red lines
    through your draft copy, and it can be a steep learning experience until you learn to slow down
    between edits.

A fast iterative approach to Rust arguably is not possible to the same extent as it would be in
Typescript. In Typescript, you get the benefit of a running Typescript server in the background to
lint errors you make before you commit to compiling. But Rust compiles to machine code, so it's more
complex (by nature and design) to reap the same benefits.

### Some Add-ons for Better Rust DX

Any language's developer experience improves in the long run with time invested. But there are tools
to make the initial DX a less steep adjustment:

-   **CodeLLDB debugger**: A VSCode extension that you'll need to implement with a `launch.json`
    file in the root of your repo. It's definitely saved me time! It brings more incremental
    compilation to the dev process, so you're often only having to wait for it to compile your most
    recently saved .rs code changes, rather than compiling everything all over again from top to
    bottom. It also brings the same compile-time checking, so you can fix your errors and bring a
    more 'one and done' approach to Cargo builds (meaning less artefacts to clean out too).

-   **rust-analyzer**: Part of the official rust-lang repository and runs in several IDEs beyond
    VSCode. I've yet to really get it working for myself (I'm fully accepting this is a skill
    issue), but it promises error checking and code completion closer to real-time feedback.

### Cleaning Out Cargo Artefacts ⚠️

I mentioned it briefly above, but it pays to be explicit here. Keep an eye when running `cargo`
commands like `build`, `run`, `test`, `check` and `bench`. All of these commands can wind up with
more artefacts building up in your target folder and you can wind up with a repository taking up
over 10 gigs of storage space on your hard disk for an app that runs no longer than ~15,000 lines of
code.

In other words: You're better off cleaning out Cargo artefacts. Run the `cargo clean` command in the
same directory as your `Cargo.toml` file regularly.

## Error Handling

There are two parts to comparing error handling here: Philosophy and Syntax.

Click here for actual syntax-related differences between TS and Rust.

As far as philosophy (and the practical implications): If you're familiar with Erlang and Elixir
school-of-thought on error handling, then Rust treating errors as values - and not exceptions like
Typescript/Javascript - will feel right at home.

I touched on the differences in philosophy in the previous section, but it's worth mentioning the
practical implications of the different approaches. It's going to sound like I'm in heavily favour
of treating errors as values (even though I owe a lot to Javascript):

1. **Performance**:

    - Rust: Zero cost for successful task outcomes. Bringing in error handling doesn't use more
      memory when no errors occur.
    - TS/JS: Handling errors as exceptions can potentially (not always) use more memory. Although
      let's be clear that Javascript engines have come to a point where the cost is often minimal.

2. **Error Scope**:

    - Rust: Errors stay local unless explicitly pushed up the call stack.
    - TS/JS: Errors can jump into the global environment if not caught and this can affect your
      entire app.

3. **Runtime Robustness**:

    - Rust: The program can continue running even if an error occurs in a thread or process.
    - TS/JS: Unhandled errors (as exceptions) risk bringing your entire app to a halt.

4. **Error Types**:
    - Rust: Strongly typed errors mean you have to handle error and success cases explicitly.
    - TS/JS: You can use your own custom, untyped errors which IS nice flexibility and makes for a
      faster sandbox experience, but can lead to edge cases failing silently both in dev and
      production.

At the end of the day, only you can be the judge of your own developer experience.

In Typescript and Javascript, you can often decide to either wrap multiple async tasks inside a
`try-catch` block when you're confident those tasks will return without fail. And there's always the
option to separate concerns when things don't go to plan. Or use `.then()` Promise callback chains
instead! It's a flexible work routine.

This choice can be less intuitive and more didactic in Rust, particularly because of the strong
error-typing in function signatures. A brief example using divide by zero just to paint the picture:

TypeScript:

```typescript
function divideNumbers(a: number, b: number): number {
	if (b === 0) {
		throw new Error('Cannot divide by zero');
	}
	return a / b;
}

try {
	const result = divideNumbers(10, 0);
	console.log(result);
} catch (error) {
	console.error('An error occurred:', error.message);
}
```

Rust:

```rust
fn divide_numbers(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide_numbers(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("An error occurred: {}", error),
    }
}
```

Go to the sub-section above to get used to the syntax around this workflow in handling error and
success outcomes as explicit values.

### Sidenote: Proposal for new error-handling syntax in Javascript

I did find a proposal to bring in a new operator to ECMAScript that blends the schools of thought
above into one [here](https://github.com/arthurfiorette/proposal-safe-assignment-operator). It is
potentially a nice addition to the JS developer experience, but it is still fundamentally working
within Javascript's runtime where errors are exceptions.

## No Garbage Collector

The approach in memory management is the main difference to get to grips with in Rust. Unlike
Javascript, there's no garbage collector.

Keep in mind that Rust is a compiled language (like Typescript), and enforces **ownership and
borrowing rules** at compile time. These rules are Rust's memory-safe guarantee; most of your
journey starts by working within that guarantee, so that you can later spot (for yourself, your
domain and your end-user) when and where to work around the guarantee without breaking it.

We'll touch upon ownership and borrowing briefly in the next section, but I'd argue it's more
important to get comfortable with Rust inside your IDE early on in your journey. Later on, we can
get deep into concepts and the shift in mindset to Rust memory management.

## Rust's Ownership Model and Concurrency vs JavaScript's Async Approach

I feel it's beneficial to get straight to practicing Rust code and seeing some common ground between
Rustdev and webdev in the section immediately after this, rather than trying to take in big,
sweeping changes of concept all at once. But just know Rust's ownership and borrowing rules - as
stated before - are the fundamental change in mindset you're taking on board in the long term.

In the short term, my advice is still the same: just find common ground between syntax, operators
and types - i.e. get your feet under the table and feel at home in your Rust dev environment first -
and observe how ownership and borrowing works in your code.

I'll deliberately keep this last part of the 'Differences' chapter as brief as possible.

### Ownership and Borrowing

Rust's ownership system means the compiler makes assumptions about what is and isn't possible at
compile time, eliminating the chance of data races. Javascript relies on you to manage shared state
and relies on runtime checks.

### Threading

Rust is a truly multithreaded language, while Javascript uses a single-threaded Event Loop. If you
need more background on the event loop, task queues and microtask queues there are great videos on
it here.

### Async/Await Runtime Cost

Because Javascript's async/await is built on top of Promises and the Event Loop, you do pay some
runtime costs in terms of object allocation and task scheduling. In comparison, Rust's async/await
is zero-cost. How?

If you're ever done game development in something like Godot, the idea of state machines may be
familiar. The generated machine code from compiled Async/await statements in Rust is as efficient as
hand-written state machines; the compiler knows exactly what state needs to be preserved between
await points. The end result is zero additional runtime cost.

### Cancellation and Blocking

It's hard to summarise the difference between writing in a language where asynchronous tasks were "a
thing" from the language's birth, compared to being brought in after the language was launched. But
async/await is a great example of how you don't need to bring in external dependencies or write any
manual helper functions to do true thread cancellation and blocking with Rust's built-in features
(the Drop trait in particular below).

Here's an example of what I'd write in Rust:

```rust
fn main() {
    let handle = thread::spawn(|| {
        (1..=5).for_each(|i| {
            println!("Thread: count {}", i);
            thread::sleep(Duration::from_secs(1));
        });
    });

    // This actually blocks the main thread
    thread::sleep(Duration::from_secs(2));

    // Cancellation happens when handle's execution scope ends
    drop(handle);

    println!("Main: Done");
}
```

Compared to what I'd write in Typescript:

```typescript
function sleep(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

async function countToFive(): Promise<void> {
	for (let i = 1; i <= 5; i++) {
		console.log(`Count: ${i}`);
		await sleep(1000);
	}
}

let cancel = false;

const countPromise = (async () => {
	try {
		await countToFive();
	} catch (e) {
		if (cancel) {
			console.log('Cancelled');
		} else {
			throw e;
		}
	}
})();

/* What you might be used to doing in webdev even if
it doesn't truly block the main thread */

setTimeout(() => {
	cancel = true;
	console.log('Trying to cancel...');
}, 2000);

console.log('Main: Done');
```

I only realised how convenient this was by just getting to writing code, and in turn I started to
see the trade-offs and compromises I'd gotten used to making in more mature languages that pre-dated
async/await (I will always have a soft spot for Python's asyncio, for example, which is still a
great library and language for the right domain).

I would suggest not dwelling on the deeper implications of the theory above for now, but it's your
choice.

In the next section, you can choose to either get immediately stuck into Rust and gain a deeper
insight into the above through practice... or you can keep on with the analytical approach if that's
your preference!

# Feeling at Home in Rust

## 1. Practice Functional Rust and Functional TS

## 2. Grind Through a Rust Developer Map

## Two Good Books for Further Reading
