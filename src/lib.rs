#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate rand;
extern crate rand_isaac;

use std::sync::Arc;
use tokio::sync::Mutex;
use webrtc::peer_connection::RTCPeerConnection;

use rand::prelude::*;

pub mod backtracking;
pub mod chp_2;
pub mod chp_3;
pub mod chp_4;
pub mod chp_6;
pub mod cryptographic_algos;
pub mod fun_stuff;
pub mod hackers_delight;
pub fn random_u64() -> u64 {
    let mut result: u64 = 0;
    let mut buffer: [u8; 8] = [0; 8];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut buffer);
    for &buf in buffer.iter() {
        result <<= 8;
        result |= buf as u64;
    }
    result
}

#[macro_use]
lazy_static! {
    static ref PEER_CONNECTION_MUTEX: Arc<Mutex<Option<Arc<RTCPeerConnection>>>> =
        Arc::new(Mutex::new(None));
}

//pub mod hackers_delight;

/*
macro_rules! define_c {
    ($name:ident, $bits:expr, $type:ident) => {define_c!(#[doc=""], $name, $bits, $type);};
    (#[$doc:meta], $name:ident, $bits:expr, $type:ident) => {
        #[$doc]
        #[allow(non_camel_case_types)]
        #[derive(Default, Clone, Copy, Debug)]
        pub struct $name($type);

        #[$doc]
        impl $name {
            pub const MAX: Self = $name(((1 as $type) << ($bits - 1)) - 1);
            pub const MIN: Self = $name(-((1 as $type) << ($bits - 1)));

            fn mask(self) -> Self {
                if (self.0 & (1<<($bits-1)) ) == 0 {
                    $name(self.0 & ( ((1 as $type) << $bits).overflowing_sub(1).0))
                } else {
                    $name(self.0 | !( ((1 as $type) << $bits).overflowing_sub(1).0))
                }
            }
        }

        implement_common!($name, $bits, $type);
    }
}


macro_rules! implement_common {
    return None;
}

not_impl! { i8 i16 }
define_c!(#[doc="The 1-bit unsigned integer type."], c1, 2, i8);
define_c!(#[doc="The 2-bit unsigned integer type."], c2, 3, i8);
define_c!(#[doc="The 3-bit unsigned integer type."], c3, 4, i8);
define_c!(#[doc="The 4-bit unsigned integer type."], c4, 5, i8);
define_c!(#[doc="The 5-bit unsigned integer type."], c5, 6, i8);
define_c!(#[doc="The 8-bit unsigned integer type."], c8, 9, i16);
define_c!(#[doc="The 6-bit unsigned integer type."], c6, 7, i8);



implement_into!([c1, c2, c3, c4, c5, c6], i8);
implement_into!([c1, c2, c3, c4, c5, c6], i64);
implement_into!([c8,c9,c10,c11,c12,c13,c14], i16);
implement_from!(c6, [c1, c2, c3, c4, c5]);

*/
