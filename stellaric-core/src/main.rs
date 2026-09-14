use stellaric_core::vector::Vector3;
use stellaric_core::physics::Body;
use stellaric_core::physics::{_AU, _SOLAR_MASS};

fn main() {
  let dt = 0.01;
// G = 1.0
let mut a = Body::new(1.0, Vector3{x: -0.5, y: 0.0, z: 0.0}, Vector3{x: 0.0, y: -0.7071067811865476, z: 0.0});
let mut b = Body::new(1.0, Vector3{x:  0.5, y: 0.0, z: 0.0}, Vector3{x: 0.0, y:  0.7071067811865476, z: 0.0});

 

  for _i in 0..21 {

  println!("A's position is -> {:?}, A's Velocity is -> {:?}", a.position, a.velocity);
  println!("B's position is -> {:?}, B's Velocity is -> {:?}", b.position, b.velocity);
  print!("\n");

  let a_acc = a.acc_vec(&b);
  let b_acc = b.acc_vec(&a);
  
  a.update_body(a_acc, dt);
  b.update_body(b_acc, dt);
  
  }



}