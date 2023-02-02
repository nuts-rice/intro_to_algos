mod midpoint {
    use cgmath::Vector3;
    use rand::rngs::StdRng;
    use rand::{random, Rng, RngCore};

    struct HeightMap {
        grid: Vector3,
        exponent: u32,
    }

    pub fn generate_terrain(heightmap: HeightMap) -> HeightMap {
        let resolution = 2.pow(heightmap.exponent) - 1;
        println!("creating heightmap of {} by {}", resolution);
    }
}
