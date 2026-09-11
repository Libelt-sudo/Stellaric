use crate::vector::Vector3;


const G: f64 = 6.674e-11;           // m³ kg⁻¹ s⁻²
const _AU: f64 = 1.496e11;           // metres (Earth–Sun distance)
const _SOLAR_RADIUS: f64 = 6.957e8;  // metres
const _EARTH_RADIUS: f64 = 6.371e6;  // metres
const _SOLAR_MASS: f64 = 1.989e30;   // kilograms

struct Body { 
  mass: f64,
  pos: Vector3
}

impl Body { 

  fn calc_distance(&self, other: &Body) -> f64{
    // Distance between the two objects centers of mass
    ((other.pos.x - self.pos.x).powi(2) + (other.pos.y - self.pos.y).powi(2) + (other.pos.z - self.pos.z).powi(2)).sqrt()
  }

  fn calc_gravitational_force(&self, other: &Body) -> f64{
    let r = self.calc_distance(other);
    
    let force = (G * self.mass * other.mass) / r.powi(2);

    force
    
  }   
}



#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    const TOL: f64 = 1e-15;

    fn body_at(x: f64, y: f64, z: f64) -> Body {
        Body { mass: 1.0, pos: Vector3 { x, y, z }}
    }

    #[test]
    fn identical_positions_give_exactly_zero() {
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
        let a = body_at(_AU, 0.0, 0.0);
        let b = body_at(_AU + 3.0, 4.0, 0.0);
        assert_relative_eq!(a.calc_distance(&b), 5.0, max_relative = TOL);
    }
}