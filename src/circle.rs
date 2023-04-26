use crate::traits::{Area, Border, PrintDetails};

#[derive(Debug)]
pub struct Circle {
  pub r: f64
}

impl Area for Circle {
  fn area(&self) -> f64 {
     std::f64::consts::PI*self.r * self.r
  }
}

impl Border for Circle {
  fn border(&self) -> f64 {
    std::f64::consts::PI * self.r * 2.0
  }
}

impl PrintDetails<Circle> for Circle {
  fn printall(&self) {
    let area = self.area();
    let border = self.border();
    println!("Area: {}, Border: {}", area, border);
  }
}
