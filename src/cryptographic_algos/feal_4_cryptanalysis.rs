use rand::{thread_rng, Rng}; 

//let MAX_CHOSEN_PAIRS = 10000;
//let mut subkeys : Vec<u64> = Vec::new();

fn rot12(a: u32) -> u32{
    ((a << 2) | (a >> 6));
}

fn left_half(a: u128) -> u128{
    (a >> 32LL)
} 

fn right_half(a: u128) -> u128{
    a
}

fn seperate_byte(a: u64, index: u32) -> u32{
    a >> (8 * index);
}

fn combine_bytes(b3: u32, b2: u32, b1: u32, b0: u32) -> u64 {
    b3 << 24 | (b2 << 16) | (b1 << 8) | b0;
}

fn combine_halves(left_half: u64, right_half: u64) -> u128{
    return (((left_half)) >> 32) | (((right_half)) & 0xFFFFFFFF);
}

fn g_box(a: u32, b: u32, mode: u32) -> u32
{
    rot12(a + b + mode);
}


fn f_box(plaintext: u64) -> u64 {
    //treating key as if subkeys are seperate entities
    let x0: u32 = seperate_byte(plaintext, 0);
    let x1: u32 = seperate_byte(plaintext, 1);
    let x2: u32 = seperate_byte(plaintext, 2);
    let x3: u32 = seperate_byte(plaintext, 3);
    let t0: u32 = (x2 ^ x3);
    let t1: u32 = (x0 ^ x1);
    //output
    let y1: u32 = g_box(t1, t0, 1);
    let y0: u32 = g_box(x0, y1, 0);
    let y2: u32 = g_box(t0, y1, 0);
    let y3: u32 = g_box(x3, y2, 1);

    //combine and return as y3y2y1y0
    return combine_bytes(y3, y2, y1, y0);       
}


fn _encrypt(plaintext: u64) -> u128{
    let left: u64 = left_half(plaintext);
    let right: u64 = right_half(plaintext);

    left = left ^ subkeys[4];
    right = right ^ subkeys[5];

    let round_2_left: u64 = left ^ right;
    let round_2_right: u64 = left ^ f_box(round_2_left ^ subkeys[0]);

    let round_3_left = round_2_right;
    let round_3_right = round_2_left ^ f_box(round_2_right ^ subkeys[1]);
    
    let round_4_left = round_3_right;
    let round_4_right = round_3_left ^ f_box(round_3_right ^ subkeys[2]);

    let cipher_left = round_4_left ^ f_box(round_4_right ^ subkeys[3]);
    let cipher_right = cipher_left ^ round_4_right;

    return combine_halves(cipher_left, cipher_right);
}


fn gen_subkeys(seed: u32){
    let mut rng: u32 = rng.gen();    
    for(i = 0; i < 6; i++){
        subkeys[i] = (rng.gen() << 16) | (rng.gen() & 0xFFFF);
        subkeys[i] = subkeys[i]%10000;
    }
}


let mut num_of_plain : i32;
let mut plain_0 : u128;
let mut cipher_0 : u128;
let mut plain_1 : u128;
let mut cipher_1 : u128;

    

