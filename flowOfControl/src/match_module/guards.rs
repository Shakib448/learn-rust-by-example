// Guards
// A match guard can be added to filter the arm.

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

pub fn run () {
    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit", t),
    }

    let number : u8 = 4;
    match number {
        i if i == 0 => println!("zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
    }
}