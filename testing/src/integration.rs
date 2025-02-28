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