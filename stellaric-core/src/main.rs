use stellaric_core::vector::Vector3;

fn main() {
  let v1 = Vector3::new(10.0, 20.0, 0.0);
  let v2 = Vector3::new(-5.0, 10.0, 0.0);

  println!("v1 + v2 = {:?} ", v1 + v2);
  println!("v1 - v2 = {:?} ", v1 - v2);

  println!("v1 * 2 = {:?} ", v1 * 2.0);
  println!("v2 * 2 = {:?} ", v2 * 2.0);

  println!("v1 . v2 = {:?} ", v1.dot(v2));
  println!("v1 x v2 = {:?} ", v1.cross(v2));
}