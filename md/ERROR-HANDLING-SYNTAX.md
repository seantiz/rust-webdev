[Go Back to Main Section](./README.md)
<br>

# Error Handling Syntax: Rust vs TypeScript/JavaScript

Here are some error-handling syntax difference to get to grips with in Rust when coming from web development.

_Please keep in mind we use some import statements at the very end that include Rust namespace syntax like `std::num::ParseIntError` for importing a `ParseIntError` type into your Rust module. We haven't covered Rust paths and namespaces before now._

## Function Signatures

These might slow you down the most when starting out with Rust. Typescript function returns are usually only explicitly typed in success cases; in Rust, a `Result` type in a function signature can resolve to either a successful outcome or an error:

```rust
fn might_fail() -> Result<SuccessType, ErrorType> {
// fn body
}
```

## Error Propagation

In TypeScript, you're likely used to `try-catch` blocks for throwing and catching errors through your callback sequence:

```typescript
async function algorithm(args: any) {
  try {
    const firstResult = await firstTask(args);
    const finalResult = await nextTask(firstResult);
  } catch (error) {
    console.log(error)
  }
}


// Elsewhere in your app

async function nextTask(input: number): Promise<number> {
    try {
        const x = await divideAsync(10, 2);
        const y = await divideAsync(x, 3);
        return y;
    } catch (error) {
        throw error;
    }
}
```
In this setup above, errors from firstTask (not shown above) or nextTask will be caught in the algorithm function. The nextTask function also has its own error handling, where you could choose to log more fine-grained error messages if you wanted.

Rust uses a different approach with the Result type and the `?` operator:

```rust
fn algorithm(args: Args) -> Result<f64, String> {
    let first_result = first_task(args)?;
    let final_result = next_task(first_result)?;
    Ok(final_result)
}

// Elsewhere in your app

fn next_task(input: f64) -> Result<f64, String> {
    let x = divide(input, 2.0)?;
    let y = divide(x, 3.0)?;
    Ok(y)
}
```

The `?` at the end of a line means 'if there's an error, immediately return it'.

If any step fails in first_task (not shown above) or next_task, the error is automatically passed back to top level wherever algorithm() was called.

The key differences are Typescript's flexibility can handle errors at different levels of your code (but can also lead to functions silently failing and going unnoticed); Rust makes you deal with possible errors at each step and forces you to think about errors more.

There is some flexibility to Rust errors (as we'll see below) but that flexibility tends to stay localised.


## More on Error Handling Syntax

Rust uses pattern matching with `match` or methods like `unwrap()`, `expect()`, `?` operator:

```rust
// Using match
match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}

// Using unwrap (panics on error, use with caution)
let result = divide(10.0, 2.0).unwrap();

// Using ? operator (propagates error)
fn operation() -> Result<f64, String> {
    let result = divide(10.0, 2.0)?;
    Ok(result * 2.0)
}
```

## When You Want More Flexible Errors in Rust

I previously wrote (in the main section) that Typescript held the advantage on letting you quickly instance and re-use custom errors. But that's an oversimplication.

You may want to keep it at that oversimplication for now and avoid any kind of decision fatigue in Rust, as a newer (modern?) language with a relatively smaller standard library. Once you start learning the error types you can import into your Rust modules, you can get flexible at a local level. Here are some examples below.

### Shaping Custom Errors

You do have rapid-fire options for custom error objects in TS/JS (from JS's standard `Error` to custom interfaces you can write to extend `Error` and even framework-specific error objects like `error` in SvelteJS), but you can also shape your own errors with enums in Rust:


```rust
enum MathError {
    DivisionByZero,
    Overflow,
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}
```

### Using error types that conform to the same Error trait

Another path to take in Rust is the ability to handle multiple error types or the `Box<dyn Error>` trait object. `Box<dyn Error>` is a way to return any error type that implements the `Error` trait:

```rust
fn read_and_parse() -> Result<i32, Box<dyn Error>> {
    let mut file = File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn main() {
    match read_and_parse() {
        Ok(n) => println!("The number is: {}", n),
        Err(e) => println!("An error occurred: {}", e),
    }
}
```

In the above example, read_and_parse() can return error that conforms to several error types. For example:

- Using `std::io::Error` to return an error from the file operation
- Using `std::num::ParseIntError` to return an error from parsing the string to an integer

Both these error types implement the `Error` trait, so they can be boxed and returned as Box<dyn Error>. We've once again used the `?` operator, so if an error occurs then it's automatically boxed and returned.

Why would you want bring this added complexity into your error handling?

It's useful when you don't know all possible error types at compile time or when you're working with libraries that return their own error types. The trade-off is that you lose some type information at compile time, but you gain flexibility in error handling.


[Go Back to Main Section](./README.md)
