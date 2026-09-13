use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl Vector3 {
  pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3 {x: x, y: y, z: z}
  }
}

impl Add for Vector3 {
  type Output = Vector3;

  fn add(self, other: Vector3) -> Vector3{
    Vector3 {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}

impl AddAssign for Vector3 {
  fn add_assign(&mut self, other: Vector3){
      self.x += other.x;
      self.y += other.y;
      self.z += other.z;
  }
}

impl Sub for Vector3 {
  type Output = Vector3;

  fn sub(self, other: Vector3) -> Vector3{
    Vector3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}

impl Mul<f64> for Vector3 {
  type Output = Vector3;

  fn mul(self, scalar: f64) -> Vector3{
    Vector3 {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar
    }
  }
}

impl Div<f64> for Vector3 {
  type Output = Vector3;

  fn div(self, scalar: f64) -> Vector3{
    Vector3 {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar
    }
  }
}



impl Vector3 {
  pub fn dot(self, other: Vector3) -> f64{
    let product: f64 = (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
    product
  }

  pub fn cross(self, other: Vector3) -> Vector3{
    Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }
}