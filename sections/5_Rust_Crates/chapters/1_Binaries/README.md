# Binaries

## Binaries

We're going to build a library, but it's useful to have an executable to test our our code.
Create a `/bin` directory within `/src`:

```shell
mkdir src/bin
```

And add a `main.rs` file to it:

```shell
touch src/bin/main.rs
```

We'll add the following code to it:

```rust
fn main() {
    println!("Hello, Mandy!");
}
```

## Check

We can check all the code within project is conceptually valid with:

```shell
cargo check
```

## Build

We can build the project by running the following command:

```shell
cargo build
```

We can build a binary optimised for release using the `--release` flag:

```shell
cargo build --release
```

Release builds are slower to compile, but they run faster.
This is because they're compiled with optimisations enabled, which takes the compiler more processing to evaluate for accuracy, and then to apply.
The compiler will also inline functions and remove unused code.

In general, you should use the default (debug) build for development, and use the release build for production.

## Run

We can run the project by running the following command:

```shell
cargo run
```

If we add multiple binaries to our project, we can specify which one we want to run using the `--bin` flag:

```shell
cargo run --bin main
```

To build and run the program in release mode, we can use the `--release` flag:

```shell
cargo run --bin main --release
```
