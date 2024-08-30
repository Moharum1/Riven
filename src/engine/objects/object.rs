use std::rc::Rc;
use crate::engine::base::interval::Interval;
use crate::engine::base::point::Point3;
use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;

/// A struct representing a record of a hit in ray tracing.
#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    /// The point at which the hit occurred.
    pub point: Point3,
    /// The normal vector at the hit point.
    pub normal: Vector3,
    /// The parameter `t` at which the hit occurred.
    pub t: f32,
    /// A boolean indicating whether the hit was on the front face.
    pub front_face: bool,
}

/// A struct representing a list of objects that can be hit by rays.
pub struct HitList {
    /// A vector of reference-counted objects that implement the `Object` trait.
    pub objects: Vec<Rc<dyn Object>>,
}

/// A trait representing an object that can be hit by a ray.
pub trait Object {
    /// Determines if a ray hits the object within a given range.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to test for intersection.
    /// * 'ray_t' An interval containing max and min t
    /// * `rec` - A mutable reference to a `HitRecord` to store hit information.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the ray hit the object.
    fn hit(&self, ray: &Ray, ray_t : Interval, rec: &mut HitRecord) -> bool;
}

impl HitList {
    /// Creates a new, empty `HitList`.
    ///
    /// # Returns
    ///
    /// A new instance of `HitList`.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Adds an object to the `HitList`.
    ///
    /// # Arguments
    ///
    /// * `object` - A reference-counted object that implements the `Object` trait.
    pub fn add(&mut self, object: Rc<dyn Object>) {
        self.objects.push(object);
    }
}

impl Object for HitList {
    /// Determines if a ray hits any object in the `HitList` within a given range.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray to test for intersection.
    /// * `t_min` - The minimum value of `t` for a valid hit.
    /// * `t_max` - The maximum value of `t` for a valid hit.
    /// * `rec` - A mutable reference to a `HitRecord` to store hit information.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the ray hit any object in the list.
    fn hit(&self, ray: &Ray, ray_t : Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                rec.point = temp_rec.point;
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
            }
        }

        hit_anything
    }
}

impl HitRecord {
    /// Sets the face normal of the hit record based on the ray and outward normal.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray that hit the object.
    /// * `outward_normal` - The normal vector pointing outward from the hit point.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}