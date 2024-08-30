use std::rc::Rc;
use crate::engine::base::point::Point3;
use crate::engine::base::ray::Ray;
use crate::engine::base::vector::Vector3;

#[derive(Clone, Copy, Default)]
pub struct HitRecord{
    pub point: Point3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
}

pub struct HitList{
    pub objects: Vec<Rc<dyn Object>>,
}

pub trait Object{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}


impl HitList{
    pub fn new() -> Self{
        Self{
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Object>){
        self.objects.push(object);
    }
}

impl Object for HitList {
     fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool{
         let mut temp_rec = HitRecord::default();
         let mut hit_anything = false;
         let mut closest_so_far = t_max;

         for object in self.objects.iter(){
             if object.hit(ray, t_min, closest_so_far, &mut temp_rec){
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


impl HitRecord{
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3){
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
}

