use crate::engine::base::interval::Interval;
use crate::engine::base::point::Point3;
use crate::engine::base::ray::Ray;

// Implementations for Axis-Aligned Bounding Box
#[derive(Clone)]
pub struct AABB{
    x : Interval,
    y : Interval,
    z : Interval
}

impl AABB{

    pub fn default() -> Self{
        Self{
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
        }
    }
    pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> Self {

        let (xl, yl, zl) = Self::pad_to_min(x, y, z);

        Self{
            x : xl,
            y : yl,
            z : zl
        }
    }

    fn pad_to_min(x : Interval, y : Interval, z : Interval) -> (Interval, Interval, Interval) {
        let delta = 0.0001;
        let mut xl : Interval = x.to_owned();
        let mut yl : Interval = y.to_owned();
        let mut zl : Interval = z.to_owned();

        if x.size() < delta {
            xl = x.expand(delta);
        }

        if y.size() < delta{
            yl  = y.expand(delta);
        }

        if z.size() < delta{
            zl = z.expand(delta);
        }

        return (x, y, z);
    }

    pub fn from_points(point1: Point3, point2 : Point3) -> Self {
        Self{
            x : if point1.x <= point2.x {
                Interval::new(point1.x, point2.x)
            } else { Interval::new(point2.x, point1.x)},

            y : if point1.y <= point2.y {
            Interval::new(point1.y, point2.y)
            } else { Interval::new(point2.y, point1.y)},

            z : if point1.z <= point2.z {
            Interval::new(point1.z, point2.z)
            } else { Interval::new(point2.z, point1.z)},
        }
    }

    pub fn from_aabb(box1 : AABB, box2 : AABB) -> AABB {

        let x = Interval::from_interval(box1.x, box2.x);
        let y = Interval::from_interval(box1.y, box2.y);
        let z = Interval::from_interval(box1.z, box2.z);

        let (xl, yl, zl) = Self::pad_to_min(x, y, z);

        Self{
            x : xl,
            y : yl,
            z : zl
        }
    }

    pub fn get_axis_interval(&self, n : i32) -> Interval{
         match n{
            1 => self.y.clone(),
            2 => self.z.clone(),
            _ => self.x.clone()
        }
    }

    pub fn hit(&self, ray : &Ray,  ray_t: &mut Interval) -> bool{
        for axis in 0..3{
            let ax = self.get_axis_interval(axis);

            let inv_d = 1.0 / ray.direction[axis];
            let mut t0 = (ax.min - ray.origin[axis]) * inv_d;
            let mut t1 = (ax.max - ray.origin[axis]) * inv_d;

            if t0 < t1 {
                ray_t.min = ray_t.min.max(t0);  // Update t_min
                ray_t.max = ray_t.max.min(t1);  // Update t_max
            }else{
                ray_t.min = ray_t.min.max(t1);  // Update t_min
                ray_t.max = ray_t.max.min(t0);  // Update t_max
            }

            // Exit early if there's no hit
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

}


