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
        let k_priv_a: BigUint = rng.gen_biguint(1000) % &p;
        let k_priv_b: BigUint = rng.gen_biguint(1000) % &p;
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

mod cryptopals_5_34 {

    use session_types::*;
    type key = Box<Vec<u8>>;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + 'static>>;
    type ProtocolSend = Recv<Box<[u8]>, Send<Result<()>, Var<Z>>>;
    type ProtocolRecieve = Send<Box<[u8]>, Choose<Var<Z>, Var<Z>>>;
    //server will recieve message, key and send (c_1, c_2) as aes
    type ProtocolServer = Recv<key, Choose<Rec<ProtocolInner>, Eps>>;
    type ProtocolInner = Offer<ProtocolSend, Offer<ProtocolRecieve, Eps>>;
    //what we assume alice and bob to be...unless there's an advesary!
    type ProtocolClient = <ProtocolServer as HasDual>::Dual;

    fn protocol_handshake(_c: Chan<(), ProtocolServer>) {
        unimplemented!()
    }
}

mod elgaml {

    fn elgaml() {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn diffie_hellman_test() {
        unimplemented!()
    }
}
