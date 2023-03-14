# Setup

We'll create the minimal file structure for a `Rust` project using the `Cargo` package manager.

## Add cargo.toml

Create a new `Cargo` project:

```shell
cargo init --name mandy
```

This will generate a [`Cargo.toml`](./Cargo.toml) file containing the project metadata.
And a `src` directory containing a `main.rs` file.

## Check

We can check the project by running the following command:

```shell
cargo check
```

This is a useful way of checking that code is conceptually valid when we don't want to actually run it yet.
This is much faster than an actual build.

## Build

We can build the project by running the following command:

```shell
cargo build
```

## Run

We can run the project by running the following command:

```shell
cargo run
```

If our source code has been changed since the last build, this will also trigger a build before running the code.

You should see `"Hello, world!"` printed to the console.

## Add .gitignore

I'm going to add a [`.gitignore`](./.gitignore) file to my project.
`Python` projects use a lot of files that we don't want to commit to `git`, so we can use a template to generate a `.gitignore` file for us.

I'm going to pull the code from https://www.toptal.com/developers/gitignore/api/rust into a `.gitignore` file using the `curl` command:

```shell
curl -L https://www.toptal.com/developers/gitignore/api/rust > .gitignore
```

Alternatively, you can go to [gitignore.io](https://gitignore.io/) and generate a template more tailored to your needs.
