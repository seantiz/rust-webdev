[Go Back to Main Section](./README.md)

TypeScript and Rust both use static typing, but they approach it in different ways:

1. **Type Inference:** Arguably Rust has the stronger type inference system, as you get get away without
   explicitly declaring types for everything except function signatures. Whether you'd want to skip
   explicit typing is a different matter. It often saves time to be explicit with data types, to
   reap the rewards of 'zero-effort' data validation throughout your pipeline later.

2. **Numbers:** Typescript groups all numeric values (except BigInt) into a single `number` primitive
   type, where Rust is a lower-level language and makes a distinction between integers and floating
   point numbers as primitives.

3. **Unit vs Void:** When you want a function to simply return without data in Typescript/Javascript,
   you use `void`. In Rust, a similar function would return unit; the unit type is an empty tuple
   `()`

4. **Null and Undefined vs Option<T>:** A Typescript function can also return `null` or `undefined`,
   especially when you need to handle errors or failed callbacks within asynchronous tasks. The
   closest equivalent to this in Rust is `Option<T>`

5. **Interfaces vs Traits:** You've likely come across the age-old question "Is it better to use types
   or interfaces?" in Typescript. There's no such debate in Rust: Traits are to shaping structs in
   Rust as Types/Interfaces are to shaping objects in Typescript.

6. **Never Type:** Type `never` is to Typescript what `!` is to Rust in an oversimplified way, but
   they're not exact equivalents and context matters a lot here.

7. **Type Safety:** Typescript's enforcement can be bypassed in any number of ways; from changing your
   `tsconfig` settings, to comments in your module script and even type assertions. Rust's
   enforcement is, by default, more strict and harder to walk around - though we'll see later this
   is a needed skill to get around Rust's type safety when you want your threads and processes to
   inform one another in meaningful ways.

[Go Back to Main Section](./README.md)