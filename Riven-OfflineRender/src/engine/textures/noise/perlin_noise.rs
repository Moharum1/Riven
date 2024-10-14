use image::imageops::filter3x3;
use crate::engine::base::constants::constants::{random_float, ranged_random_int};
use crate::engine::base::point::Point3;
use crate::engine::base::vector::Vector3;

#[derive(Clone)]
pub struct PerlinNoise {
    point_count: usize,
    rand_vec: Vec<Vector3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        const POINT_COUNT: usize = 256;
        let mut rand_vec = vec![Vector3::default(); POINT_COUNT];
        let mut perm_x = vec![0; POINT_COUNT];
        let mut perm_y = vec![0; POINT_COUNT];
        let mut perm_z = vec![0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            rand_vec[i] = Vector3::create_random_vector_with_bound(-1.0, 1.0);
        }

        Self::perlin_generate_perm(&mut perm_x, POINT_COUNT as i32);
        Self::perlin_generate_perm(&mut perm_y, POINT_COUNT as i32);
        Self::perlin_generate_perm(&mut perm_z, POINT_COUNT as i32);

        PerlinNoise {
            point_count: POINT_COUNT,
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    //TODO: Unknown error in noise creation
    pub fn noise(&self, point: Point3) -> f32 {
        let u = point.x - point.x.floor();
        let  v = point.y - point.y.floor();
        let  w = point.z - point.z.floor();

        let i = point.x.floor() as usize;
        let j = point.y.floor() as usize;
        let k = point.z.floor() as usize;

        let mut c = [[[Vector3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[(
                        self.perm_x[(i + di) & 255] ^
                        self.perm_y[(j + dj) & 255] ^
                        self.perm_z[(k + dk) & 255]
                    ) as usize];
                }
            }
        }

        self.trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(&self, c: [[[Vector3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut acc = 0f32;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {

                    let weight_vec = Vector3::new(
                        u - i as f32,
                        v - j as f32 ,
                        w - k as f32
                    );

                    acc += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                         * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                         * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                         * c[i][j][k].dot(&weight_vec);
                }
            }
        }

        acc
    }

    pub(crate) fn turb(&self, point : Point3, depth : i32) -> f32 {
        let mut acc = 0.0;
        let mut temp_p = point;
        let mut weight = 1.0;

        for _ in 0..depth{
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p
        }

        acc.abs()
    }

    fn perlin_generate_perm(p: &mut Vec<i32>, n: i32) {
        for i in 0..n {
            p[i as usize] = i;
        }

        Self::permute(p, n);
    }

    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in (1..n).rev() {
            let target = ranged_random_int(0, i);
            let temp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = temp;
        }
    }
}
