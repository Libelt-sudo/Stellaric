use crate::vector::Vector3;


pub const G: f64 = 4.0 * std::f64::consts::PI * std::f64::consts::PI;         // m³ kg⁻¹ s⁻²
//pub const G: f64 = 1.0;              // m³ kg⁻¹ s⁻²
pub const AU: f64 = 1.496e11;           // metres (Earth–Sun distance)
pub const SOLAR_RADIUS: f64 = 6.957e8;  // metres
pub const EARTH_RADIUS: f64 = 6.371e6;  // metres
pub const SOLAR_MASS: f64 = 1.989e30;   // kilograms

#[derive(Debug)]
pub struct Body { 
  pub mass: f64,
  pub position: Vector3,
  pub velocity: Vector3
}

impl Body {
    pub fn new(mass: f64, position: Vector3, velocity: Vector3) -> Body{
        Body{mass: mass, position: position, velocity: velocity}
    }
}

impl Body { 

    fn calc_distance(&self, other: &Body) -> f64 {
        // Distance between the two objects centers of mass
        ((other.position.x - self.position.x).powi(2) + (other.position.y - self.position.y).powi(2) + (other.position.z - self.position.z).powi(2)).sqrt()
    }

    fn calc_gravitational_force(&self, other: &Body, r: f64) -> f64 {
        (G * self.mass * other.mass) / r.powi(2)
    }

    fn calc_direction_vector(&self, other: &Body, distance: f64) -> Vector3 {
        (other.position - self.position) / distance
    }

    pub fn update_body(&mut self, acc_vec: Vector3, dt: f64) {
        let final_vel = self.velocity + acc_vec * dt;
        self.velocity = final_vel;

        self.position += self.velocity * dt;
    }
}


pub struct Sandbox {
    pub delta_time: f64,
    pub bodies: Vec<Body>
}

impl Sandbox {
    pub fn new(delta_time: f64) -> Sandbox{
        Sandbox {bodies: vec![], delta_time: delta_time}
    }
}

impl Sandbox {
    pub fn create_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn update(&mut self) {
        
        let mut acc_vecs: Vec<Vector3> = vec![];
        let n = self.bodies.len();

        for i in 0..n {
            let mut total_acc = Vector3{x: 0.0, y: 0.0, z: 0.0};
            for j in 0..n {

                // skip if it is the same body
                if i == j{
                    continue;
                }

                let body = &self.bodies[i];
                let other = &self.bodies[j];

                // calcs distance between the two bodies
                let distance = body.calc_distance(other);
                // calcs direction unit vector
                let dir_vector = body.calc_direction_vector(other, distance);
                let g_force = body.calc_gravitational_force(other, distance);
                
                // scalar multipies dir vector with G-force to get force magnitude and direction, stored as a vector
                let foce_dir_vector = dir_vector * g_force;

                // calcs the acceleration vector with a = f / m.
                // xyz components are each divded by mass. 
                let acc_vec = foce_dir_vector / body.mass;

                // sums acceleration between all the bodies.  
                total_acc += acc_vec;
            }

            acc_vecs.push(total_acc);
        }

        for i in 0..n{
            let body = &mut self.bodies[i];
            let acc = acc_vecs[i];

            body.update_body(acc, self.delta_time);
        }  
    }
}


#[cfg(test)]
mod distance_tests {
    use super::*;
    use approx::assert_relative_eq;

    const TOL: f64 = 1e-15;

    fn body_at(x: f64, y: f64, z: f64) -> Body {
        Body { mass: 1.0, position: Vector3 { x, y, z }, velocity: Vector3 { x:0.0, y:0.0, z:0.0 }}
    }

    #[test]
    fn identical_positionitions_give_exactly_zero() {
        let a = body_at(1.5, -2.25, 3.0);
        assert_eq!(a.calc_distance(&a), 0.0);
    }

    #[test]
    fn axis_aligned_distance_is_the_coordinate_difference() {
        let a = body_at(0.0, 0.0, 0.0);
        let b = body_at(5.0, 0.0, 0.0);
        assert_relative_eq!(a.calc_distance(&b), 5.0, max_relative = TOL);
    }

    #[test]
    fn matches_pythagoras_in_3d() {
        // 1-2-2-3 quadruple: sqrt(1 + 4 + 4) == 3
        let a = body_at(0.0, 0.0, 0.0);
        let b = body_at(1.0, 2.0, 2.0);
        assert_relative_eq!(a.calc_distance(&b), 3.0, max_relative = TOL);
    }

    #[test]
    fn handles_negative_and_mixed_coordinates() {
        let a = body_at(-1.0, 2.0, -3.0);
        let b = body_at(2.0, -2.0, 9.0);
        // differences are (3, -4, 12), so the distance is 13
        assert_relative_eq!(a.calc_distance(&b), 13.0, max_relative = TOL);
    }

    #[test]
    fn works_at_astronomical_magnitudes() {
        // relative tolerance is scale-free, so the same TOL applies here
        let a = body_at(0.0, 0.0, 0.0);
        let b = body_at(3.0 * AU, 4.0 * AU, 0.0);
        assert_relative_eq!(a.calc_distance(&b), 5.0 * AU, max_relative = TOL);
    }

    
}

#[cfg(test)]
mod force_tests {
    use approx::assert_relative_eq;

    use super::*;

    const TOL: f64 = 1e-9;      // relative tolerance
    const ABS_TOL: f64 = 1e-30; // absolute, for comparisons against zero

    #[test]
    fn zero_force_with_no_mass() {
        let a = Body::new(0.0, Vector3 { x: AU, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let b = Body::new(0.0, Vector3 { x: AU * 10.0, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });

        assert_relative_eq!(a.calc_gravitational_force(&b, a.calc_distance(&b)), 0.0, epsilon = ABS_TOL);
    }

    #[test]
    fn zero_force_when_other_body_is_massless() {
        let a = Body::new(SOLAR_MASS, Vector3 { x: 0.0, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let b = Body::new(0.0, Vector3 { x: AU, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });

        assert_relative_eq!(a.calc_gravitational_force(&b, a.calc_distance(&b)), 0.0, epsilon = ABS_TOL);
    }

    #[test]
    fn unit_masses_unit_distance_equals_g() {
        let a = Body::new(1.0, Vector3 { x: 0.0, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let b = Body::new(1.0, Vector3 { x: 1.0, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });

        assert_relative_eq!(a.calc_gravitational_force(&b, a.calc_distance(&b)), G, max_relative = TOL);
    }

    #[test]
    fn doubling_distance_quarters_force() {
        let a = Body::new(SOLAR_MASS, Vector3 { x: 0.0, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let near = Body::new(SOLAR_MASS, Vector3 { x: AU, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let far = Body::new(SOLAR_MASS, Vector3 { x: AU * 2.0, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });

        let f_near = a.calc_gravitational_force(&near, a.calc_distance(&near));
        let f_far = a.calc_gravitational_force(&far, a.calc_distance(&far));

        assert_relative_eq!(f_far, f_near / 4.0, max_relative = TOL);
    }

    #[test]
    fn doubling_mass_doubles_force() {
        let a = Body::new(SOLAR_MASS, Vector3 { x: 0.0, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let light = Body::new(SOLAR_MASS, Vector3 { x: AU, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let heavy = Body::new(SOLAR_MASS * 2.0, Vector3 { x: AU, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });

        let f_light = a.calc_gravitational_force(&light, a.calc_distance(&light));
        let f_heavy = a.calc_gravitational_force(&heavy, a.calc_distance(&heavy));

        assert_relative_eq!(f_heavy, f_light * 2.0, max_relative = TOL);
    }

    #[test]
    fn force_is_symmetric_between_bodies() {
        let a = Body::new(SOLAR_MASS * 2.0, Vector3 { x: AU, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let b = Body::new(SOLAR_MASS, Vector3 { x: -AU, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });

        assert_relative_eq!(
            a.calc_gravitational_force(&b, a.calc_distance(&b)),
            b.calc_gravitational_force(&a, b.calc_distance(&a)),
            max_relative = TOL
        );
    }

    #[test]
    fn distance_is_direction_independent() {
        // same separation, different axis — force must be identical
        let origin = Body::new(SOLAR_MASS, Vector3 { x: 0.0, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let on_x = Body::new(SOLAR_MASS, Vector3 { x: AU, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let on_y = Body::new(SOLAR_MASS, Vector3 { x: 0.0, y: AU, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let on_z = Body::new(SOLAR_MASS, Vector3 { x: 0.0, y: 0.0, z: AU }, Vector3 { x:0.0, y:0.0, z:0.0 });

        let f_x = origin.calc_gravitational_force(&on_x, origin.calc_distance(&on_x));

        assert_relative_eq!(origin.calc_gravitational_force(&on_y, origin.calc_distance(&on_y)), f_x, max_relative = TOL);
        assert_relative_eq!(origin.calc_gravitational_force(&on_z, origin.calc_distance(&on_z)), f_x, max_relative = TOL);
    }

    #[test]
    fn distance_uses_euclidean_norm_on_a_3_4_5_triangle() {
        // r should be 5, not 7 (a naive sum of components)
        let a = Body::new(1.0, Vector3 { x: 0.0, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let b = Body::new(1.0, Vector3 { x: 3.0, y: 4.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });

        assert_relative_eq!(a.calc_gravitational_force(&b, a.calc_distance(&b)), G / 25.0, max_relative = TOL);
    }

    #[test]
    fn works_with_bodies_away_from_the_origin() {
        // difference vector is (1, 2, 2), so r = 3 exactly
        let a = Body::new(1.0e3, Vector3 { x: 1.0, y: 2.0, z: 3.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let b = Body::new(2.0e3, Vector3 { x: 2.0, y: 4.0, z: 5.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });

        assert_relative_eq!(
            a.calc_gravitational_force(&b, a.calc_distance(&b)),
            G * 1.0e3 * 2.0e3 / 9.0,
            max_relative = TOL
        );
    }

    #[test]
    fn handles_negative_coordinates() {
        // separation is 2 AU across the origin
        let a = Body::new(SOLAR_MASS, Vector3 { x: -AU, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });
        let b = Body::new(SOLAR_MASS, Vector3 { x: AU, y: 0.0, z: 0.0 }, Vector3 { x:0.0, y:0.0, z:0.0 });

        let expected = (G * SOLAR_MASS * SOLAR_MASS) / (2.0 * AU).powi(2);

        assert_relative_eq!(a.calc_gravitational_force(&b, a.calc_distance(&b)), expected, max_relative = TOL);
    }
}