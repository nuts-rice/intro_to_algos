use rand::prelude::*;
use rand::rngs::OsRng;
use rand_core::RngCore;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

//type Seed: Sized + Default + AsMut<[u8]>;

//fn from_seed(seed: Self::Seed) -> Self;

pub fn rand_bytes_buff(len: usize) -> Vec<u8> {
    let mut r: Vec<u8> = vec![0; len];
    OsRng.fill_bytes(r.as_mut_slice());

    r
}

pub fn from_string() {
    let mut rng: Pcg64 = Seeder::from("mango umbrella").make_rng();
    println!("{}", rng.gen::<char>());
}

#[cfg(test)]
mod test {
    use super::*;
    use rand_seeder::SipRng;
    #[test]
    fn seeder_test() {
        let mut seeder = Seeder::from("test string");
        let mut rng = seeder.make_rng::<SipRng>();
        assert_eq!(rng.next_u64(), 7267854722795183454);
    }
}
