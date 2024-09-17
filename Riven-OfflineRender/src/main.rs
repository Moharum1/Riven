use Riven_OfflineRender::engine::base::point::Point3;
use Riven_OfflineRender::engine::base::vector::Vector3;
use Riven_OfflineRender::engine::camera::rgb_camera::RGBCamera;
use Riven_OfflineRender::engine::lighting::diffuse_lighting_model::lambertian::Lambertian;
use Riven_OfflineRender::engine::lighting::diffuse_lighting_model::metal::Metal;
use Riven_OfflineRender::engine::objects::object::HitList;
use Riven_OfflineRender::engine::objects::sphere::Sphere;
use Riven_OfflineRender::util::color::Color;
use Riven_OfflineRender::util::image::Canvas;
//TODO: Multi-threading                []
//TODO: Bounding Volume Hierarchy      []
//TODO: Texture Mapping                []
//TODO: Normal Mapping                 []
//TODO: Adding light source            []
//TODO: Support for more shapes        []
//TODO: Support for more materials     []
//TODO: Support for more cameras       []
//TODO: Support for more samplers      []
//TODO: Support for more integrators   []
//TODO: Organize the codebase          []

fn main() {
    let image_width = 1200;
    let canvas = Canvas::new(image_width);

    // world
    let mut world = HitList::new();

    let ground_mat = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat));

    let mut cam = RGBCamera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 5;

    cam.vfov = 20.0;
    cam.look_from = Point3::new(10.0, 2.0, 3.0);
    cam.look_at = Point3::new(5.0, 2.0, 3.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.1;
    cam.focus_dist = 10.0;

    let mat1 = Lambertian::new(Color::new(0.0, 0.8, 0.0));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Lambertian::new(Color::new(1.0, 0.0, 0.0));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.5);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));


    cam.render(&world, canvas);
}



