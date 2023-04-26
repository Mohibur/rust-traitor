use crate::traits::{Area, Border, PrintDetails};

#[derive(Debug)]
pub struct Rectangle {
  pub x: f64,
  pub y: f64
}


impl Area for Rectangle {
  fn area(&self) -> f64 {
    return (self.x + self.y) * 2.0
  }
}

impl Border for Rectangle {
  fn border(&self) -> f64 {
    return self.x * self.y
  }
}

impl PrintDetails<Rectangle> for Rectangle {
  fn printall(&self) {
    let area = self.area();
    let border = self.border();
    println!("Area: {}, Border: {}", area, border);
  }
}
