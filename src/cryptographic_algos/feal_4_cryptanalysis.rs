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
    for(i = 0, i < 6, i++){
        subkeys[i] = (rng.gen() << 16) | (rng.gen() & 0xFFFF);
        subkeys[i] = subkeys[i]%10000;
    }
}


let mut num_of_plain : i32;
let mut plain_0 : u128;
let mut cipher_0 : u128;
let mut plain_1 : u128;
let mut cipher_1 : u128;

    
fn undo_final_op() {
    for(i = 0, i < num_of_plain, i++) {
        cipher_left_0 = left_half(cipher_0[i]);
        cipher_right_0 = right_half(cipher_0[i]) ^ cipher_left_0;
        
        cipher_left_1 = left_half(cipher_1[i]);
        cipher_right_1 = right_half(cipher_1[i]) ^ cipher_left_1;

        cipher_0[i] = combine_halves(cipher_left_0, cipher_right_0);
        cipher_1[i] = combine_halves(cipher_left_1, cipher_right_1);                      
    
    }
}

fn crack_last_round(outdiff: u64) -> u64 {
    println!("Cracking output differental of 0x{:?}", outdiff);

    let mut fake_k: u64;
    for (fake_k = 0x00000000, fakeK < 0xFFFFFFFF, fakeK++) {
        //score for when calculated differntal matches actual
        let mut score : u32;
        for (c = 0, c < num_of_plain, c++){
            let mut cipher_left: u128 = (cipher_0[c] >> 32);
            cipher_left ^= (cipher_1[c] >> 32);
            let mut cipher_right: u128 = cipher_0[c] & 0xFFFFFFFF;
            cipher_right ^= (cipher_1[c] & 0xFFFFFFFF);

            let mut Y: u64 = cipher_right;
            let mut Z: u64 = cipher_left ^ outdiff;

            let mut fake_right: u128 = cipher_0[c] & 0xFFFFFFFF;
            let mut fake_left: u128 = cipher_0[c] >> 32;
            let mut fake_right_2: u128 = cipher_1[c] & 0xFFFFFFFF;
            let mut fake_left_2: u128 = cipher_1[c] >> 32;


            let mut Y0 = fake_right;
            let mut Y1 = fake_right_2;
            
            //faked input and output
            let mut fake_in_0 = Y0 ^ fake_k;
            let mut fake_in_1 = Y1 ^ fake_k;
            let mut fake_out_0 = f_box(fake_in_0);
            let mut fake_out_1 = f_box(fake_in_1);
            let mut fake_diff = fake_out_0 ^ fake_out_1;

            if (fake_diff == Z){
                score+= 1;
            } else {
                break;
            }

            if(score == num_of_plain){
                println!("found subkey: 0x{:?}", fake_k);
                fake_k;
            }
        }
        println!("didnt find subkey");
        0;
    }
}

fn chosen_plain(diff: u128) {
    println!("generating {} chosen-plaintext pairs
             using input differental of 0x{:?}", num_of_plain, diff);
    for(c: u32 = 0; c < num_of_plain; c++){
        let mut plain_0[c]: u128 = (rand.rng() & 0xFFFF) << 48; 
        plain_0[c] += (rand.rng() & 0xFFFF) << 32;
        plain_0[c] += (rand.rng() & 0xFFFF) << 16;
        plain_0[c] += (rand.rng() & 0xFFFF);

        cipher_0[c] = _encrypt(plain_0[c]);
        //other half of pair uses input differental 
        plain_1[c] = plain_0[c] ^ diff;
        cipher_1[c] = _encrypt(plain_1[c]);
    }
}
