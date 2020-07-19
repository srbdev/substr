# substr

A helper crate for substring functionality for Rust's `str`.

```toml
[dependencies]
substr = { git = "https://github.com/srbdev/substr" }
```

#### ```fn firstn(s: &str, n: usize) -> &str```

Returns the first nth characters from the input string `s`.

#### ```fn lastn(s: &str, n: usize) -> &str```

Returns the last nth characters from the input string `s`.

#### ```fn slice(s: &str, start: usize, end: usize) -> &str```

Return the substring of the input string `s` with indices [start, end).


***Note: `substr` will only work on UTF-8 sequences or may otherwise reference invalid memory or
violate the invariants communicated by the `str` type.***