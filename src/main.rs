use traits::PrintDetails;

pub mod circle;
pub mod traits;
pub mod rectangle;

fn main() {
  let circle = circle::Circle {
    r: 10.0
  };
  let rectangle = rectangle::Rectangle {
    x: 10.0,
    y: 20.0
  };
  circle.printall();
  rectangle.printall();
}
