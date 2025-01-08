/*
 * Any program requires comments, and Rust supports a few different varieties:
 * Regular comments which are ignored by the compiler:
 * // Line comment which got tot the end of the line.
 * /* Bloc comments which go to the closing delimiter. */
*/


/*
* Doc comment which are parsed into HTML library
* /// Generate library docs for the following item.
* //! Generate the library docs for the enclosing item.
*/
fn main() {
    let x = 5 + /*90 + */ 5;
    println!("The value of x is: {}", x);
}