# clowncopterize

## Clowncopterize
`clowncopterize` is a tool to make setting all `clowntown` cli arguments easier.

### Background
The `clowntown` command line argument is a well-known reliability feature that allows to hide risky features behind a flag. As reliability
is taken more and more seriously, the sprawling of `clowntown` flags is becoming an issue for our savvy engineer who are ending up writing
command lines that goes beyond our cherrished 80-char limits.

To make people's life easier, here comes `clowncopterize`! With a single line added to your program, you can achieve the apex of reliability
by providing the almighty `--clowncopterize` argument which will set all your `--clowntown-X` flags to true.

### Requirements
`clowncopterize` is an attribute macro to apply to a `clap` struct to make it easier to set all those --clowntown-X flags with a single `--clowncopterize` flag.

### Usage
Wrap you clap Parser struct with
```rust
#[clowncopterize::clowncopterize]
```

This macro must be above the derive one.

#### Example
```rust
#[clowncopterize::clowncopterize]
#[derive(Parser, Debug)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(long)]
    clowntown_this: bool,

    /// lists test values
    #[arg(long)]
    clowntown_that: bool,
}
```


License: MIT
