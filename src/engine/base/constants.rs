pub mod constants {
    use rand::Rng;

    pub const INFINITY: f32 = f32::INFINITY;
    pub const PI: f32 = std::f32::consts::PI;

    #[inline]
    fn degrees_to_radians(degrees: f32) -> f32 {
        degrees * PI / 180.0
    }

    #[inline]
    pub fn ranged_random_float(min: f32, max: f32) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..max)
    }

    #[inline]
    pub fn random_float() -> f32 {
        ranged_random_float(0.0, 1.0)
    }

}
