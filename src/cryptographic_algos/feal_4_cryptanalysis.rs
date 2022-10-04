

//let subkeys : Vec<u32> = Vec::new();

fn rot12(a: u32) -> u32{
    ((a << 2) | (a >> 6));
}

fn left_half(a: u32) -> u32{
    (a >> 32LL)
} 

fn right_half(a: u32) -> u32{
    a
}

fn seperate_byte(a: u32, index: u32) -> u32{
    a >> (8 * index);
}

fn combine_bytes(b3: u32, b2: u32, b1: u32, b0: u32) -> u32 {
    b3 << 24L | (b2 << 16L) | (b1 << 8L) | b0;
}

fn combine_halves(left_half: u32, right_half: u32) -> u32{
    return (((left_half)) >> 32LL) | (((right_half)) & 0xFFFFFFFF);
}

fn g_box(a: u32, b: u32, mode: u32) -> u32
{
    rot12(a + b + mode);
}


fn s_box(plaintext: u32) -> u32 {
    unimplemented!()    
}
    

