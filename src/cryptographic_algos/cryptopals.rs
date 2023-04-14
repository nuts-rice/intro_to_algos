use crate::cryptographic_algos::elgaml_shanks;

mod challenge_5_34 {
    use super::*;
    use digest::Digest;
    use num_bigint::BigUint;
    use sha1::Sha1;

    #[derive(Default, Debug)]
    pub struct DH {
        pub g: BigUint,
        pub p: BigUint,
    }

    pub type Key = Box<Vec<u8>>;

    pub fn secret_to_key(s: &[u8]) -> Key {
        Box::new(Sha1::digest(s)[0..16].to_vec())
    }

    impl DH {
        pub fn new() -> Self {
            let p_hex = b"ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74\
                     020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f1437\
                     4fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7ed\
                     ee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf05\
                     98da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb\
                     9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff";
            let p = BigUint::parse_bytes(p_hex, 16).unwrap();
            DH {
                g: BigUint::new(vec![2]),
                p: (p),
            }
        }
    }
}
mod challenge_5_35 {
    use result::ResultOptionExt;
    use std::fmt::Debug;

    use super::*;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

    pub trait DHClient: Clone + Default + Debug {
        fn send(&mut self, message: &[u8]) -> Result<()>;
        fn recieve(&mut self) -> Result<Option<Vec<u8>>>;
    }
    pub trait DHClientNew: Clone + Default + Debug {
        fn send(&mut self, message: &[u8]) -> Result<()>;
        fn recieve(&mut self) -> Result<Option<Vec<u8>>>;
    }
}

mod challenge_6_41 {

    use super::*;
    use chrono::prelude::*;
    use openssl::bn::MsbOption;
    use openssl::{bn, rsa};
    use std::ops::{Add, Div, Mul};
    const SIZE: usize = 512;
    //TODO: figure out the derive trait annotation here
    //    #[derive(Clone, Debug)]
    pub struct Value(pub Box<bn::BigNum>);
    struct Oracle {
        rsa: rsa::Rsa<bn::BigNum>,
        clear: bn::BigNum,
        cipher: bn::BigNum,
    }

    impl Oracle {
        fn new() -> Self {
            let rsa = rsa::Rsa::generate(SIZE as u32);
            let mut bn = bn::BigNum::new().unwrap();
            let clear = bn.rand(SIZE as i32 - 1, MsbOption::MAYBE_ZERO, true);
            todo!()
        }
    }

    impl Add<Value> for Oracle {
        type Output = Value;

        fn add(self, b: Self::Output) -> Self::Output {
            todo!()
        }
    }

    impl Mul<Value> for Oracle {
        type Output = Value;

        fn mul(self, b: Self::Output) -> Self::Output {
            todo!()
        }
    }

    impl Div<Value> for Oracle {
        type Output = Value;

        //Blah blah blah modulo check
        fn div(self, b: Self::Output) -> Self::Output {
            todo!()
        }
    }
}

mod attack {
    macro_rules! evil {
        ($var:ident) => {
            if cfg!(feature = "attack") {
                todo!()
            } else {
                $var
            }
        };
    }
}

mod mitm_34_attack {
    use super::*;
    use result::ResultOptionExt;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

    macro_rules! evil {
        ($var:ident) => {
            if cfg!(feature = "attack") {
                todo!()
            } else {
                $var
            }
        };
    }

    pub trait MitmHandshake<T: challenge_5_35::DHClient> {
        fn handshake(client_stream: &mut T, server_stream: &mut T) -> Result<Vec<u8>>;
    }
}

#[cfg(test)]
mod test {

    fn diffie_hellman_test() {
        todo!()
    }
}
