use crate::engine::base::interval::Interval;
use crate::engine::base::ray::Ray;
use crate::engine::bounding_model::aabb::AABB;
use crate::engine::objects::hit_record::HitRecord;
use crate::engine::objects::Objects;

/// A trait representing an object that can be hit by a ray.
pub trait GeometricObject{
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
    fn hit(&self, ray: &Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB;
}



/// A struct representing a list of objects that can be hit by rays.
#[derive(Clone)]
pub struct HitList {
    /// A vector of reference-counted objects that implement the `Object` trait.
    pub objects: Vec<Objects>,
    bbox : AABB
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
            bbox: AABB::default(),
        }
    }

    /// Adds an object to the `HitList`.
    ///
    /// # Arguments
    ///
    /// * `object` - A reference-counted object that implements the `Object` trait.
    pub fn add(&mut self, object: Objects) {
        self.bbox = AABB::from_aabb(self.bbox.clone(), object.bounding_box());
        self.objects.push(object);
    }
}

impl GeometricObject for HitList {
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
    fn hit(&self, ray: &Ray, ray_t : &mut Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(ray, &mut Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                rec.point = temp_rec.point;
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
                rec.mat = temp_rec.mat.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
