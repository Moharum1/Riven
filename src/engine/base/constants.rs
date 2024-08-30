pub mod constants {
    pub const INFINITY: f32 = f32::INFINITY;
    pub const PI: f32 = std::f32::consts::PI;

    #[inline]
    fn degrees_to_radians(degrees: f32) -> f32 {
        degrees * PI / 180.0
    }

}
