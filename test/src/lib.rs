#[cfg(test)]
mod tests {

    use paramtest::{paramtest, tokio_paramtest};

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

    #[paramtest(
    test1=(None, false),
    test2=(Some("hello"), true),
    test3=(Some("world"), true),
)]
    fn has_value(input: Option<&str>, pass: bool) {
        assert_eq!(pass, input.is_some())
    }

    #[derive(PartialEq, Eq)]
    enum TestEnum {
        Variant1,
        Variant2,
    }

    #[paramtest(
    test1=(TestEnum::Variant1, false),
    test2=(TestEnum::Variant2, true),
)]
    fn variant(input: TestEnum, pass: bool) {
        assert_eq!(pass, input == TestEnum::Variant2)
    }

    #[tokio_paramtest(
    test1=(TestEnum::Variant1, false),
    test2=(TestEnum::Variant2, true),
)]
    async fn tokio_variant(input: TestEnum, pass: bool) {
        assert_eq!(pass, input == TestEnum::Variant2);
    }

    #[tokio_paramtest(
    test1=(TestEnum::Variant1, false),
    test2=(TestEnum::Variant2, true),
)]
    async fn tokio_variant_result(
        input: TestEnum,
        pass: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(pass, input == TestEnum::Variant2);

        Ok(())
    }
}
