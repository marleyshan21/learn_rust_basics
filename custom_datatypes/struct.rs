// Rust has 3 types of structs 
// 1. Tuple structs, 2. OG C Structs and 3. Unit structs

#![allow(dead_code)] //hide warning for un used code

#[derive(Debug)]
struct Person {

    name: String,
    age: u8,
}

// unit struct
struct Unit;

// tuple struct
struct Pair (i32, f32);

// Struct with 2 fields
struct Point{

    x: f32,
    y: f32,
}

// struct of a struct
struct Rectangle {


    top_left: Point,
    bottom_right: Point,

}

// Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).

fn rect_area (rect: &Rectangle) -> f32{

    let Rectangle {
        top_left: Point{ x: x1, y:y1},
        bottom_right: Point{ x:x2, y:y2}
    } = rect;

    let width = x2 - x1;
    let height = y2 - y1;

    width * height


}

fn main(){

    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};

    // debug 
    println!("{:?}", peter);

    // instantiate a point
    let point: Point = Point { x: 5.2, y: 0.4};
    let point_2: Point = Point {x: 10.3, y: 0.2};

    // access the fields
    println!("Point coordinates: {}  {} ", point.x, point.y);

    // create bottom left using point_2
    // by using .., we are essentailly saying that first field is a new value and the rest of the fields,
    // here y, can be directly copied over from point_2
    let bottom_right = Point {x: 10.3, ..point_2};
    println!("Second point coords: {} {} ", bottom_right.x, bottom_right.y);

    // destructing the point
    let Point {x: left_edge, y: top_edge} = point;

    let _rectangle = Rectangle{
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,

    };

    println!("Area of the rectangle is {}", rect_area(&_rectangle));
  



}