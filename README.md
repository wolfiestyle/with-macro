# The `with` macro

[![Build Status](https://travis-ci.org/darkstalker/with-macro.svg?branch=master)](https://travis-ci.org/darkstalker/with-macro) [![crates.io](https://meritbadge.herokuapp.com/with-macro)](https://crates.io/crates/with-macro) [![Documentation](https://docs.rs/with-macro/badge.svg)](https://docs.rs/with-macro)

This is a macro that takes an object and lets you call methods on that object without naming it.
The first argument is an expression that will be assigned to a variable in let binding. To make
that binding mutable, prepend `mut` to the expression.
Calling a function that starts with a `.` will be converted into a method call using this
variable.

The supported forms are:
- `.method(args..)`
- `let pat = .method(args..);`
- `var = .method(args..);`

Anything else will be evaluated unmodified as an expression.

## Usage
```Rust
use with_macro::with;

let vec = with! {
    mut Vec::new() =>
        .push(1)
        .push(42)
        .push(-13)
        let l = .len();
        assert_eq!(l, 3);
};

assert_eq!(vec, [1, 42, -13]);
```
