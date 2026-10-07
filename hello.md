# Creating and running a "Hello, world!" program

## 0. Check the installation

```console
$ rustc --version
$ cargo --version
```

## 1. Create the package

Go to the parent directory of the package, and run:

```console
$ cargo new hello
```

`cargo` is Rust's build tool and package manager.
It just created everything a working program needs:

```
hello
├── .git/           a git repository, initialized for you
├── .gitignore      it already ignores the target directory
├── Cargo.toml      the manifest: name, version, dependencies
└── src
    └── main.rs     the source code
```

## 2. Run it

Step into the directory and let cargo do the work:

```console
$ cd hello
$ cargo run
```

## 3. Useful cargo commands

| Command | What it does |
|---|---|
| `cargo new <name>` | creates a new package |
| `cargo run` | builds if needed, then runs |
| `cargo check` | only checks for errors, does not produce an executable — the fastest feedback |
| `cargo build` | builds into `target/debug/` |
| `cargo build --release` | optimized build into `target/release/` |
| `cargo run --release` | builds and runs the optimized version |
| `cargo clean` | deletes the `target` directory |

**Debug and release are not only about speed.** The debug build keeps the
runtime checks: an integer overflow, for example, stops the program with a clear
message, while the release build lets the value wrap around silently. During
development, always use the debug build.
