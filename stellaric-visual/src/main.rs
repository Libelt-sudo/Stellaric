use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;


use stellaric_core::physics::Sandbox;
use stellaric_core::vector::Vector3;
use stellaric_core::physics::Body;
use stellaric_core::physics::{AU, SOLAR_MASS};

fn main() {
    let dt = 0.0005; // years (~4.4 hours)

    let sun     = Body::new(1.0,      Vector3{x: 0.0,   y: 0.0, z: 0.0}, Vector3{x: 0.0, y: -0.003373, z: 0.0});
    let mercury = Body::new(1.660e-7, Vector3{x: 0.387, y: 0.0, z: 0.0}, Vector3{x: 0.0, y: 10.1002,   z: 0.0});
    let venus   = Body::new(2.448e-6, Vector3{x: 0.723, y: 0.0, z: 0.0}, Vector3{x: 0.0, y: 7.3895,    z: 0.0});
    let earth   = Body::new(3.003e-6, Vector3{x: 1.000, y: 0.0, z: 0.0}, Vector3{x: 0.0, y: 6.2832,    z: 0.0});
    let mars    = Body::new(3.227e-7, Vector3{x: 1.524, y: 0.0, z: 0.0}, Vector3{x: 0.0, y: 5.0897,    z: 0.0});
    let jupiter = Body::new(9.545e-4, Vector3{x: 5.203, y: 0.0, z: 0.0}, Vector3{x: 0.0, y: 2.7546,    z: 0.0});
    let saturn  = Body::new(2.858e-4, Vector3{x: 9.537, y: 0.0, z: 0.0}, Vector3{x: 0.0, y: 2.0346,    z: 0.0});
    let uranus  = Body::new(4.366e-5, Vector3{x: 19.19, y: 0.0, z: 0.0}, Vector3{x: 0.0, y: 1.4343,    z: 0.0});
    let neptune = Body::new(5.151e-5, Vector3{x: 30.07, y: 0.0, z: 0.0}, Vector3{x: 0.0, y: 1.1458,    z: 0.0});

    let mut system = Sandbox::new(dt);

    system.create_body(sun);
    system.create_body(mercury);
    system.create_body(venus);
    system.create_body(earth);
    system.create_body(mars);
    system.create_body(jupiter);
    system.create_body(saturn);
    system.create_body(uranus);
    system.create_body(neptune);

    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open("orbit_data.csv")
    .unwrap();
        
    if let Err(e) = writeln!(file, "step,body,x,y,z") {
        eprintln!("Couldn't write to file: {}", e);
    }

        
    for step in 0..60000 {

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

