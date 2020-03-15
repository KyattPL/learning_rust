struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f64 {
        let Point {
            x: point1x,
            y: point1y,
        } = self.top_left;
        let Point {
            x: point2x,
            y: point2y,
        } = self.bottom_right;
        let diff_x = point2x - point1x;
        let diff_y = point1y - point2y;
        diff_x * diff_y
    }
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 3.0, y: 7.0 },
        bottom_right: Point { x: 10.0, y: 2.0 },
    };
    println!("Area of the rectangle is {}", rect.rect_area());
    let square = square(&Point { x: 5.0, y: 4.3 }, 6.0);
    println!("Area of the square is {}", square.rect_area());
}

fn square(point: &Point, width: f64) -> Rectangle {
    let bottom_x = point.x + width;
    let bottom_y = point.y - width;
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y,
        },
        bottom_right: Point {
            x: bottom_x,
            y: bottom_y,
        },
    }
}
