use proc::paramtest;

#[paramtest(
    one=(1, 2),
    two=(2, 3),
    three=(100, 101),
)]
fn add_on(input: u64, output: u64) {
    assert_eq!(output, input + 1)
}

#[paramtest(
    test1=("", true),
    test2=("hello", false),
    test3=("world", false),
)]
fn empty(input: &str, pass: bool) {
    assert_eq!(pass, input.is_empty())
}
