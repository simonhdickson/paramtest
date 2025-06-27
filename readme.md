# paramtest

With `paramtest`, you can define multiple sets of input values for a single test function, and each set will be run as a separate test case.

## Example

Here's how you can use `paramtest` in your tests:

```rust
use proc::paramtest;

#[paramtest(
    one=(1, 2),
    two=(2, 3),
    hundred=(100, 101),
)]
fn add_one(input: u64, output: u64) {
    assert_eq!(output, input + 1)
}
```

This is effectively the same as writing the following:

```rust
#[test]
fn add_one_one() {
    add_on(1, 2);
}

#[test]
fn add_one_two() {
    add_on(2, 3);
}

#[test]
fn add_one_hundred() {
    add_on(100, 101);
}

fn add_one(input: u64, output: u64) {
    assert_eq!(output, input + 1)
}
```

Each `#[paramtest(...)]` attribute defines named test cases with their own arguments. The macro will generate a separate test for each case, making your tests concise and easy to maintain.

## Usage

1. Add`paramtest` crate as a dependency in your `Cargo.toml`.
2. Annotate your test functions with `#[paramtest(...)]` and provide named cases with argument tuples.

## License

MIT
