/*
Associated functions & Methods

Some functions are connected to a particular type. These come in two forms: associated functions,
and methods. Associated functions are functions that are defined on a type generally, while
methods are associated functions that are called on a particular instance of a type
*/

struct Point {
    x: f64,
    y: f64,
}


impl Point {
    fn origin() -> Point {
        Point { x: 1.0, y: 1.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}


struct Rectangle {
    p1: Point,
    p2: Point,
}


impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = &self.p1;
        let Point { x: x2, y: y2 } = &self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = &self.p1;
        let Point { x: x2, y: y2 } = &self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

pub fn run() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());


    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
}