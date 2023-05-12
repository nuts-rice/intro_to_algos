mod midpoint {
    use anyhow::Error;
    use cgmath::Vector3;
    use rand::Rng;

    struct HeightMap {
        grid: Vector3<i32>,
        exponent: i32,
        spread_rate: f64,
    }
    impl HeightMap {
        pub fn new(exponent: i32, spread_rate: f64) -> HeightMap {
            HeightMap {
                grid: Vector3 {
                    x: exponent,
                    y: exponent,
                    z: exponent,
                },
                exponent: (exponent),
                spread_rate: (spread_rate),
            }
        }

        pub fn generate_terrain(&mut self) -> Result<(), Error> {
            let mut _rng = rand::thread_rng();
            let resolution = 2_i32.pow(self.exponent.try_into().unwrap()) - 1;
            println!("creating heightmap of {} by {}", resolution, resolution);
            while resolution > 1 {
                for i in (resolution / 2..self.grid.x - 1).step_by(resolution.try_into().unwrap()) {
                    for j in
                        (resolution / 2..self.grid.y - 1).step_by(resolution.try_into().unwrap())
                    {
                        let _square_average = self.grid.x - resolution / 2 + self.grid.x
                            - resolution / 2
                            + self.grid.x
                            + resolution / 2
                            + self.grid.x
                            + resolution / 2 / 4;
                        println!("square average of {}", _square_average);
                        let displacement =
                            (_rng.gen_range(0.0..1.0) - 0.5) * resolution as f64 * self.spread_rate;
                    }
                }
            }
            Ok(())
        }
    }
}
