# substr

![Rust](https://github.com/srbdev/substr/workflows/Rust/badge.svg?branch=master&event=push)

A helper crate for substring functionality for Rust's `str` type.

### Usage

Add the following to your project's `Cargo.toml` to include `substr` as a dependency from GitHub:

```toml
[dependencies]
substr = { git = "https://github.com/srbdev/substr" }
```

or from [crates.io](https://crates.io/):

```toml
[dependencies]
substrs = "0.1.0"
```
<br /> 

#### ```fn firstn(s: &str, n: usize) -> &str```

Returns the first nth characters from the input string `s`.

#### ```fn lastn(s: &str, n: usize) -> &str```

Returns the last nth characters from the input string `s`.

#### ```fn slice(s: &str, start: usize, end: usize) -> &str```

Return the substring of the input string `s` with indices [start, end).

<br />
<br /> 

***Note: `substr` will only work on UTF-8 sequences or may otherwise reference invalid memory or
violate the invariants communicated by the `str` type.***
