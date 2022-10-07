// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point {x: xt, y: yt}, 
        bottom_right: Point {x: xb, y: yb}, 
    } = rect;
    return (xb - xt)*(yt - yb)
}

fn square(p: Point, num: f32) -> Rectangle {
    let Point { x, y } = p;
    Rectangle {
        top_left: p, 
        bottom_right: Point {x: x + num, y: y + num}, 
    }
}

fn main() {
    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: 0.0, y: 2.0 },
        bottom_right: Point { x: 2.0, y: 0.0 },
    };

    println!("Rect area {:?}", rect_area(rectangle));
    println!("Square {:?}", square(Point { x: 0.0, y: 0.0 }, 5.0));
}
