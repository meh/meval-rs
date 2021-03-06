# meval-rs

[![Build Status](https://travis-ci.org/rekka/meval-rs.svg?branch=master)](https://travis-ci.org/rekka/meval-rs)

This [Rust] crate provides a simple math expression parsing and evaluation. Its main goal is to
be convenient to use, while allowing for some flexibility. Currently works only with `f64`
types.

## Documentation

[Full API documentation](http://rekka.github.io/meval-rs/meval/index.html)

## Installation

Simply add the corresponding entry to your `Cargo.toml` dependency list:

```toml
[dependencies]
meval = "0.0.4"
```

and add this to your crate root:

```rust
extern crate meval;
```

## Simple examples

```rust
extern crate meval;

fn main() {
    let r = meval::eval_str("1 + 2").unwrap();

    println!("1 + 2 = {}", r);
}
```

Need to define a Rust function from an expression? No problem, use [`Expr`](http://rekka.github.io/meval-rs/meval/struct.Expr.html)
for this and more:

```rust
extern crate meval;

fn main() {
    let expr = meval::Expr::from_str("sin(pi * x)").unwrap();
    let func = expr.bind("x").unwrap();

    let vs: Vec<_> = (0..100+1).map(|i| func(i as f64 / 100.)).collect();

    println!("sin(pi * x), 0 <= x <= 1: {:?}", vs);
}
```

[`Expr::bind`](http://rekka.github.io/meval-rs/meval/struct.Expr.html#method.bind) returns a boxed closure that is slightly less
convenient than an unboxed closure since `Box<Fn(f64) -> f64>` does not implement `FnOnce`,
`Fn` or `FnMut`. So to use it directly as a function argument where a closure is expected, it
has to be manually dereferenced:

```rust
let func = meval::Expr::from_str("x").unwrap().bind("x").unwrap();
let r = Some(2.).map(&*func);
```

Custom constants and functions? Define a context!

```rust
let y = 1.;
let expr = meval::Expr::from_str("phi(-2 * zeta + x)").unwrap();

// create a context with function definitions and variables
let ctx = (meval::CustomFunc("phi", |x| x + y), ("zeta", -1.));
// bind function with builtins as well as custom context
let func = expr.bind_with_context((meval::builtin(), ctx), "x").unwrap();
assert_eq!(func(2.), -2. * -1. + 2. + 1.);
```

For functions with 2, 3, and N variables use `CustomFunc2`, `CustomFunc3` and `CustomFuncN`
respectively.

If you need a custom function depending on mutable parameters, you will need to use a
[`Cell`](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html):

```rust
use std::cell::Cell;
let y = Cell::new(0.);
let expr = meval::Expr::from_str("phi(x)").unwrap();

let ctx = meval::CustomFunc("phi", |x| x + y.get());
let func = expr.bind_with_context(ctx, "x").unwrap();
assert_eq!(func(2.), 2.);
y.set(3.);
assert_eq!(func(2.), 5.);
```

## Supported expressions

`meval` supports basic mathematical operations on floating point numbers:

- binary operators: `+`, `-`, `*`, `/`, `^` (power)
- unary operators: `+`, `-`

It supports custom variables like `x`, `weight`, `C_0`, etc. A variable must start with
`[a-zA-Z_]` and can contain only `[a-zA-Z0-9_]`.

Build-ins (given by context `meval::builtin()`) currently supported:

- functions implemented using functions of the same name in [Rust std library][std-float]:

    - `sqrt`, `abs`
    - `exp`, `ln`
    - `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`
    - `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`
    - `floor`, `ceil`, `round`
    - `signum`

- other functions:

    - `max(x, ...)`, `min(x, ...)`: maximum and minimumum of 1 or more numbers

- constants:

    - `pi`
    - `e`

## Related projects

This is a toy project of mine for learning Rust, and to be hopefully useful when writing
command line scripts. For other similar projects see:

- [rodolf0/tox](https://github.com/rodolf0/tox)

[Rust]: https://www.rust-lang.org/
[std-float]: http://doc.rust-lang.org/stable/std/primitive.f64.html

## License

This project is dual-licensed under the Unlicense and MIT licenses.

You may use this code under the terms of either license.
