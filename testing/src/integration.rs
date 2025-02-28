/*
Integration testing
Unit tests are testing one module in isolation at a time: they're small and can test private code. Integration tests are external to your crate and use only its public interface in the same way any other code would. Their purpose is to test that many parts of your library work correctly together.

Cargo looks for integration tests in tests directory next to src.
*/

// Define this in a crate called `adder`.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}


/*
Development dependencies
Sometimes there is a need to have dependencies for tests (or examples, or benchmarks) only. Such dependencies are added to Cargo.toml in the [dev-dependencies] section. These dependencies are not propagated to other packages which depend on this package.

One such example is pretty_assertions, which extends standard assert_eq! and assert_ne! macros, to provide colorful diff.
File Cargo.toml:

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
*/