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
    use openssl::hash::{hash, MessageDigest};
    use openssl::symm::{decrypt, encrypt, Cipher};
    use rand::Rng;
    use result::ResultOptionExt;
    use std::fmt::Debug;

    use super::*;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

    //session based handshake member fns
    pub trait DHClient: Clone + Default + Debug {
        fn send(&mut self, message: &[u8]) -> Result<()>;
        fn recieve(&mut self) -> Result<Option<Vec<u8>>>;
    }
    pub trait DHClientNew: Clone + Default + Debug {
        fn send(&mut self, message: &[u8]) -> Result<()>;
        fn recieve(&mut self) -> Result<Option<Vec<u8>>>;
    }

    pub trait DHClientEncrypt: DHClient {
        fn recieve_aes(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>> {
            self.recieve()?
                .as_ref()
                .map(|message| self.decrypt(message, key))
                .invert()
        }

        fn decrypt(&mut self, message: &[u8], key: &[u8]) -> Result<Vec<u8>> {
            let aes_cbc = Cipher::aes_128_cbc();
            let iv = vec![0u32; aes_cbc.iv_len().unwrap()];
            //TODO: make this not suck
            //            let mut rng = rand::thread_rng();
            //            rng.fill(&mut iv);
            let (iv, ciphertxt) = message.split_at(aes_cbc.iv_len().unwrap());
            let result = decrypt(aes_cbc, key, Some(iv), ciphertxt)?;
            Ok(result)
        }
        fn encrypt(&mut self, message: &[u8], key: &[u8]) -> Result<Vec<u8>> {
            let aes_cbc = Cipher::aes_128_cbc();
            let mut iv = vec![0u8; aes_cbc.iv_len().unwrap()];
            let mut ciphertxt = encrypt(aes_cbc, key, Some(&iv), message)?;
            iv.extend(ciphertxt);
            Ok(iv)
        }
    }
}

mod challenge_6_41 {

    use super::*;
    use libc::sysconf;
    use openssl::bn::MsbOption;
    use openssl::{bn, rsa::Rsa};
    use session_types::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    const INTERVAL: Duration = Duration::from_secs(60 * 60);
    type Hashes = Arc<Mutex<HashMap<String, SystemTime>>>;

    type OracleServer<A> = Recv<A, Send<A, Eps>>;
    type OracleClient<A> = <OracleServer<A> as HasDual>::Dual;

    // fn new() -> Self {
    //     let rsa = rsa::Rsa::generate(SIZE as u32);
    //     let mut bn = bn::BigNum::new().unwrap();
    //     let clear = bn.rand(SIZE as i32 - 1, MsbOption::MAYBE_ZERO, true);
    //     todo!()
    // }

    fn decrypt_blob(blob: &[u8], rsa: &Rsa<openssl::pkey::Private>) -> Vec<u8> {
        let mut decrypted_blob = vec![0; rsa.size() as usize];
        let _ = rsa.private_decrypt(blob, &mut decrypted_blob, openssl::rsa::Padding::PKCS1);
        decrypted_blob
    }

    fn oracle<A: std::marker::Send + 'static + Sized>(
        c: Chan<(), OracleServer<A>>,
        rsa: &Rsa<openssl::pkey::Private>,
        hashes: Hashes,
        blob: &Vec<A>,
    ) {
        let (c, blob) = c.recv();
        //TODO: transmute or push this to something good for session type
        todo!()
        // let decrypted_blob = decrypt_blob(&blob, &rsa);
        //         let hash = format!("{:x}", md5::compute(&decrypted_blob));
        //         let embed = SystemTime::now();
        //         let mut hashes = hashes.lock().unwrap();
        //         hashes.insert(hash, embed);
        //         timeout(&mut hashes);

        // c.send(decrypted_blob).unwrap().close();
    }

    fn timeout(hashes: &mut HashMap<String, SystemTime>) {
        let now = SystemTime::now();
        hashes.retain(|_, &mut timestamp| now.duration_since(timestamp).unwrap() < INTERVAL);
    }

    fn server<A: std::marker::Send + Sized + 'static>(c: Chan<(), OracleServer<A>>) {
        let (oracle_chan, blob) = c.recv();

        todo!()
    }

    //impl Add<Value> for Oracle {
    //    type Output = Value;

    //    fn add(self, b: Self::Output) -> Self::Output {
    //        todo!()
    //    }
    //}

    //impl Mul<Value> for Oracle {
    //    type Output = Value;

    //    fn mul(self, b: Self::Output) -> Self::Output {
    //        todo!()
    //    }
    //}

    //impl Div<Value> for Oracle {
    //    type Output = Value;

    //    //Blah blah blah modulo check
    //    fn div(self, b: Self::Output) -> Self::Output {
    //        todo!()
    //    }
    //}
}

mod attack_6_41 {
    use super::*;
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

//mod mitm_34_attack {
//    use super::{challenge_5_34::DH, *};
//    use challenge_5_34::Key;
//    use openssl::{
//        bn::BigNum,
//        dh::{self, Dh},
//        ssl::ErrorCode,
//    };
//    use result::ResultOptionExt;
//    use session_types::*;
//    use std::thread;
//    type Result<T> =
//        std::result::Result<T, Box<dyn std::error::Error + std::marker::Send + Sync + 'static>>;
//    //TODO: just use openssl for this one
//    type Server<A> = Recv<A, Send<A, Eps>>;
//    type Client = <Server<A> as HasDual>::Dual;

//    macro_rules! evil {
//        ($var:ident) => {
//            if cfg!(feature = "attack_5_34") {
//                todo!()
//            } else {
//                $var
//            }
//        };
//    }

//    pub trait MitmHandshake<T: challenge_5_35::DHClient> {
//        fn handshake(c: Chan<(), Server<A>>) -> Result<Vec<BigNum>>;
//    }

//    pub trait MitmClientServer<T: challenge_5_35::DHClient> {
//        //KISS here and not rolling own session based traits
//        type Mitm: MitmHandshake<T>;
//    }

//    fn generate<A: std::marker::Send + Copy + 'static>(
//        c: Chan<(), Server<A>>,
//        params: &Vec<A>,
//    ) -> Dh<A> {
//        let dh = DH::new();
//        todo!();
//    }

//    #[derive(Debug, Copy, Clone)]
//    struct FakePublicKey(Box<BigNum>); //Q
//    #[derive(Debug, Copy, Clone)]
//    struct Params(BigNum, BigNum); //P, G

//    impl<T: challenge_5_35::DHClient, A> MitmHandshake<T> for FakePublicKey {
//        fn handshake(c: Chan<(), Server<A>>) -> Result<Vec<BigNum>> {
//            // let mut c = c.enter();
//            let mut c = {
//                //let params: Params =  c.recv();

//                todo!()
//            };
//        }
//    }
//}

#[cfg(test)]
mod test {

    fn diffie_hellman_test() {
        todo!()
    }
}
