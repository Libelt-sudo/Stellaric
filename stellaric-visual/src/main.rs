use stellaric_core::physics::*;
use stellaric_core::vector::*;
use kiss3d::prelude::*;
use std::collections::VecDeque;
use std::vec;

const TRAIL_LEN: usize = 500;


#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Kiss3d: cube").await;
    let mut camera = OrbitCamera3d::new_with_frustum(45.0, 50.0, 2000.0, Vec3::new(500.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0));
    let mut scene = SceneNode3d::empty();

    

    scene
        .add_light(Light::point(100.0))
        .set_position(Vec3::new(0.0, 5.0, -2.0));

    let dt = 0.05; // years (~4.4 hours)

    let mut system = Sandbox::new(dt);

    let center = Body::new(10000.0, Vector3{x:   0.0,      y:   0.0,      z: 0.0}, Vector3{x: 0.0, y: 0.0, z: 0.0});
    let p1  = Body::new(1.0, Vector3{x:  100.0,   y:    0.0,   z: 0.0}, Vector3{x:  0.0,   y: 10.0,   z: 0.0});
    let p2  = Body::new(1.0, Vector3{x:    0.0,   y:  150.0,   z: 0.0}, Vector3{x: -8.165, y:  0.0,   z: 0.0});
    let p3  = Body::new(1.0, Vector3{x: -200.0,   y:    0.0,   z: 0.0}, Vector3{x:  0.0,   y: -7.071, z: 0.0});
    let p4  = Body::new(1.0, Vector3{x:    0.0,   y: -300.0,   z: 0.0}, Vector3{x:  5.774, y:  0.0,   z: 0.0});
    let p5  = Body::new(1.0, Vector3{x:  282.843, y:  282.843, z: 0.0}, Vector3{x: -3.536, y:  3.536, z: 0.0});
    let p6  = Body::new(1.0, Vector3{x: -353.553, y:  353.553, z: 0.0}, Vector3{x: -3.162, y: -3.162, z: 0.0});
    let p7  = Body::new(1.0, Vector3{x:  250.0,   y:    0.0,   z: 0.0}, Vector3{x:  0.0,   y:  5.477, z: 3.162});
    let p8  = Body::new(1.0, Vector3{x:    0.0,   y:  350.0,   z: 0.0}, Vector3{x:  5.345, y:  0.0,   z: 0.0});
    let p9  = Body::new(1.0, Vector3{x:  212.132, y: -212.132, z: 0.0}, Vector3{x:  3.266, y:  3.266, z: 0.0});
    let p10 = Body::new(1.0, Vector3{x: -318.198, y: -318.198, z: 0.0}, Vector3{x:  4.243, y: -4.243, z: 0.0});

    system.create_body(center);
    system.create_body(p1);
    system.create_body(p2);
    system.create_body(p3);
    system.create_body(p4);
    system.create_body(p5);
    system.create_body(p6);
    system.create_body(p7);
    system.create_body(p8);
    system.create_body(p9);
    system.create_body(p10);




    let mut spheres: Vec<SceneNode3d> = vec![];

    for _ in 0..system.bodies.len() {
        spheres.push(scene.add_sphere(1.0).set_color(GREEN));
    }


    
    let mut trails: Vec<VecDeque<Vec3>> = vec![VecDeque::with_capacity(TRAIL_LEN); system.bodies.len()];

    while window.render_3d(&mut scene, &mut camera).await {
        for i in 0..system.bodies.len() {
            spheres[i].set_position(Vec3::new(system.bodies[i].position.x as f32, system.bodies[i].position.y as f32, system.bodies[i].position.z as f32));

            let pos = spheres[i].position();
            if trails[i].len() == TRAIL_LEN {
                trails[i].pop_front();
            }
            trails[i].push_back(pos);

            // draw the trail, fading out towards the oldest segment
            let n = trails[i].len();
            for j in 1..n {
                let mut color = GREEN;
                color.a = j as f32 / n as f32;
                window.draw_line(trails[i][j - 1], trails[i][j], color, 2.0, false);
            }
        }



        system.update();
    }
    
    
}
