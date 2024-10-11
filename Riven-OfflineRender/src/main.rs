use Riven_OfflineRender::engine::base::constants::constants::{random_float, ranged_random_float};
use Riven_OfflineRender::engine::base::point::Point3;
use Riven_OfflineRender::engine::base::vector::Vector3;
use Riven_OfflineRender::engine::camera::rgb_camera::RGBCamera;
use Riven_OfflineRender::engine::lighting::diffuse_lighting_model::dielectric::Dielectric;
use Riven_OfflineRender::engine::lighting::diffuse_lighting_model::lambertian::Lambertian;
use Riven_OfflineRender::engine::lighting::diffuse_lighting_model::metal::Metal;
use Riven_OfflineRender::engine::objects::object::HitList;
use Riven_OfflineRender::engine::bounding_model::bvh::BvhNode;
use Riven_OfflineRender::engine::objects::Objects::{BVH, List};
use Riven_OfflineRender::engine::textures::{ TextureType};
use Riven_OfflineRender::engine::objects::sphere::Sphere;
use Riven_OfflineRender::engine::textures::chess_board_texture::ChessBoardTexture;
use Riven_OfflineRender::engine::textures::image_texture::ImageTexture;
use Riven_OfflineRender::engine::textures::solid_color::SolidColor;
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


fn bouncing_spheres(){
    let image_width = 1200;
    let canvas = Canvas::new(image_width);

    // world
    let mut world = HitList::new();

    let texture = ChessBoardTexture::new(
        0.32,
        SolidColor::from_rgb(0.2, 0.3, 0.1),
        SolidColor::from_rgb(0.9, 0.9, 0.9)
    );
    let ground_mat = Lambertian::from_texture(texture);
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat));

    for i in -11..11{
        for j in -11..11{
            let chose_mat = random_float();
            let center = Point3::new(i as f32 + 0.9 * random_float(), 0.2, j as f32 + 0.9 * random_float());

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9{
                if chose_mat < 0.8{
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo.r, albedo.g, albedo.b);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
                else if chose_mat < 0.95{
                    // metal
                    let albedo = Color::random();
                    let fuzz = ranged_random_float(0.0, 0.5);
                    let sphere_material = Metal::new(albedo.r , albedo.g, albedo.b, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
                else{
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let night_earth: TextureType = ImageTexture::new("G:/Projects/Rust/Riven/Riven-OfflineRender/src/engine/textures/nightEarth.jpg");
    let mat2 = Lambertian::from_texture(night_earth);
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Metal::new(0.7, 0.6, 0.5, 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

    // let enhanced_world = BvhNode::from_world(world);

    let mut cam = RGBCamera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 10;
    cam.max_depth = 5;

    cam.vfov = 20.0;
    cam.look_from = Point3::new(13.0, 2.0, 3.0);
    cam.look_at = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.1;
    cam.focus_dist = 10.0;

    cam.render(&List(world), canvas);
    // cam.render(&enhanced_world, canvas)
}

fn three_spheres() {
    let image_width = 1200;
    let canvas = Canvas::new(image_width);

    // world
    let mut world = HitList::new();


    let texture = ChessBoardTexture::new(
        0.32,
        SolidColor::from_rgb(0.2, 0.3, 0.1),
        SolidColor::from_rgb(0.9, 0.9, 0.9)
    );
    let ground_mat = Lambertian::from_texture(texture);
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

    let mat1 = Lambertian::new(0.0, 0.8, 0.0);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1.clone()));
    //
    let mat2 = Lambertian::new(1.0, 0.0, 0.0);
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));
    //


    let night_earth: TextureType = ImageTexture::new("G:/Projects/Rust/Riven/Riven-OfflineRender/src/engine/textures/nightEarth.jpg");
    let mat3 = Lambertian::from_texture(night_earth);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

    // let bvh = BvhNode::from_world(world);


    cam.render(&List(world), canvas);
}


fn main() {
    bouncing_spheres()
}

#[cfg(test)]
mod tests {
    use Riven_OfflineRender::engine::objects::Objects::List;
    use Riven_OfflineRender::engine::textures::chess_board_texture::ChessBoardTexture;
    use Riven_OfflineRender::engine::textures::solid_color::SolidColor;
    use super::*;

    #[test]
    fn test_scene_with_three_spheres() {
        let image_width = 1200;
        let canvas = Canvas::new(image_width);

        // world
        let mut world = HitList::new();


        let texture = ChessBoardTexture::new(
            0.32,
            SolidColor::from_rgb(0.2, 0.3, 0.1),
            SolidColor::from_rgb(0.9, 0.9, 0.9)
        );
        let ground_mat = Lambertian::from_texture(texture);
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

        let mat1 = Lambertian::new(0.0, 0.8, 0.0);
        world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1.clone()));
        //
        let mat2 = Lambertian::new(1.0, 0.0, 0.0);
        world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));
        //
        let mat3 = Metal::new(0.7, 0.6, 0.5, 1.5);
        world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

        // let bvh = BvhNode::from_world(world);


        cam.render(&List(world), canvas);
    }

    #[test]
    fn bouncing_spheres(){
        let image_width = 1200;
        let canvas = Canvas::new(image_width);

        // world
        let mut world = HitList::new();

        let ground_mat = Lambertian::new(0.5, 0.5, 0.5);
        world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat));

        for i in -11..11{
            for j in -11..11{
                let chose_mat = random_float();
                let center = Point3::new(i as f32 + 0.9 * random_float(), 0.2, j as f32 + 0.9 * random_float());

                if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9{
                    if chose_mat < 0.8{
                        // diffuse
                        let albedo = Color::random() * Color::random();
                        let sphere_material = Lambertian::new(albedo.r, albedo.g, albedo.b);
                        world.add(Sphere::new(center, 0.2, sphere_material));
                    }
                    else if chose_mat < 0.95{
                        // metal
                        let albedo = Color::random();
                        let fuzz = ranged_random_float(0.0, 0.5);
                        let sphere_material = Metal::new(albedo.r , albedo.g, albedo.b, fuzz);
                        world.add(Sphere::new(center, 0.2, sphere_material));
                    }
                    else{
                        // glass
                        let sphere_material = Dielectric::new(1.5);
                        world.add(Sphere::new(center, 0.2, sphere_material));
                    }
                }
            }
        }

        let mat1 = Dielectric::new(1.5);
        world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));

        let mat2 = Lambertian::new(0.4, 0.2, 0.1);
        world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));

        let mat3 = Metal::new(0.7, 0.6, 0.5, 0.0);
        world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

        let enhanced_world = BvhNode::from_world(world);

        let mut cam = RGBCamera::default();
        cam.aspect_ratio = 16.0 / 9.0;
        cam.image_width = 1200;
        cam.samples_per_pixel = 100;
        cam.max_depth = 50;

        cam.vfov = 20.0;
        cam.look_from = Point3::new(13.0, 2.0, 3.0);
        cam.look_at = Point3::new(0.0, 0.0, 0.0);
        cam.vup = Vector3::new(0.0, 1.0, 0.0);

        cam.defocus_angle = 0.1;
        cam.focus_dist = 10.0;

        // cam.render(&List(world), canvas);
        cam.render(&enhanced_world, canvas)
    }
}



