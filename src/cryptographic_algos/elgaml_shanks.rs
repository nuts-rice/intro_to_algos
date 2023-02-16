//Parameters:
//p: a "large" prime
//g: interger having large prime order in F_p*

use anyhow::Error;
use num_bigint_dig::{BigUint, RandBigInt};
use rand::prelude::*;
mod diffie_hellman {
    use super::*;
    #[derive(Clone, Debug, Default, PartialEq)]
    struct SharedSecretPair {
        a: BigUint,
        b: BigUint,
    }

    fn diffie_hellman(p: BigUint, g: u32) -> Result<SharedSecretPair, Error> {
        let mut rng = thread_rng();
        let k_priv_a: BigUint = rng.gen_biguint(1000) % p.clone();
        let k_priv_b: BigUint = rng.gen_biguint(1000) % p.clone();
        let k_pub_a = BigUint::from(g).modpow(&k_priv_a, &p);
        let k_pub_b = BigUint::from(g).modpow(&k_priv_b, &p);
        let k_session_1 = k_pub_b.modpow(&k_priv_a, &p);
        let k_session_2 = k_pub_a.modpow(&k_priv_b, &p);
        assert_eq!(k_session_1, k_session_2);
        let shared_pair = SharedSecretPair {
            a: k_session_1,
            b: k_session_2,
        };
        Ok(shared_pair)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn diffie_hellman_test() {
        unimplemented!()
    }
}
