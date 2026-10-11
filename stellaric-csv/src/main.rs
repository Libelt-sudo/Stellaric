use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;


use stellaric_core::physics::Sandbox;
use stellaric_core::vector::Vector3;
use stellaric_core::physics::Body;
use stellaric_core::physics::{AU, SOLAR_MASS};

fn main() {
    let dt = 0.0005; // years (~4.4 hours)

    let star   = Body::new(1.0,   Vector3{x: 0.0, y: 0.0, z: 0.0}, Vector3{x: 0.0, y: -0.001, z: 0.0});
    let planet = Body::new(0.001, Vector3{x: 1.0, y: 0.0, z: 0.0}, Vector3{x: 0.0, y:  1.0,   z: 0.0});

    let mut system = Sandbox::new(dt);

    system.create_body(star);
    system.create_body(planet);

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("orbit_data.csv")
    .unwrap();
        
    if let Err(e) = writeln!(file, "step,body,x,y,z") {
        eprintln!("Couldn't write to file: {}", e);
    }

        
    for step in 0..20000 {

        for (i, body) in system.bodies.iter().enumerate() {
            let p = &body.position;
            let data = format!("{},{},{},{},{}", step, i, p.x, p.y, p.z);

            if let Err(e) = writeln!(file, "{data}") {
                    eprintln!("Couldn't write to file: {}", e);
                }

            
            }

        system.update();
    }

}

