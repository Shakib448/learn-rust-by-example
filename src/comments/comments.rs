mod my_comments {
    pub fn comments() {
        // This is a single line comments
        /*
        * This is the multiline comments
        * Yes multiline
        */

        let x = 5 + /* 90 + */ 5;
        println!("Is `x` 10 or 100? x = {}", x);
    }
}