
pub trait Area {
  fn area(&self) -> f64;
}

pub trait Border {
  fn border(&self) -> f64;
}


pub trait PrintDetails<D: Area + Border> {
  fn printall(&self);
}
