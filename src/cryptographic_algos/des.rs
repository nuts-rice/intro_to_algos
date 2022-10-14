use super::tables;

#[derive(Clone)]
pub struct Des {
    pub(crate) keys: [u64; 16],
}

fn delta_swap(a: u64, delta: u64, mask: u64) -> u64 {
    let b = (a ^ (a >> delta)) & mask;
    a ^ b ^ (b << delta)
}

fn pc1(mut key: u64) -> u64 {
    key = delta_swap(key, 2, 0x3333000033330000);
    key = delta_swap(key, 4, 0x0f0f0f0f00000000);
    key = delta_swap(key, 8, 0x009a000a00a200a8);
    key = delta_swap(key, 16, 0x00006c6c0000cccc);
    key = delta_swap(key, 1, 0x1045500500550550);
    key = delta_swap(key, 32, 0x00000000f0f0f5fa);
    key = delta_swap(key, 8, 0x00550055006a00aa);
    key = delta_swap(key, 2, 0x0000333330000300);
    key & 0xFFFFFFFFFFFFFF00
}

pub fn gen_round_keys(key: u64) -> [u64; 16] {
    let mut keys: [u64; 16] = [0; 16];

    let mut c_d: u64 = permute(key, 64, &tables::PC1);

    let data_mask = (1 << 55) | (1 << 27);
    let zero_mask = (0xFF << 56) | (1 << 28);

    for (i, &num_shifts) in tables::LSHIFTS.iter().enumerate() {
        for _ in 0..num_shifts {
            let data: u64 = (c_d & data_mask) >> 27;
            c_d = (c_d << 1) & !zero_mask | data;
        }
        keys[i] = permute(c_d, 56, &tables::PC2);
    }

    keys
}

fn permute(block: u64, block_size: u8, permutation: &[u8]) -> u64 {
    let mut res: u64 = 0;

    for &pos in permutation.iter() {
        res <<= 1;
        let p: u8 = block_size - pos;
        res |= (block & (1 << p)) >> p;
    }
    res
}

fn f(block: u64, key: u64) -> u64 {
    let mut res: u64 = 0;
    let temp: u64 = permute(block, 32, &tables::E) ^ key;
    let mask: u64 = 0b111111;
    for (i, sbox) in tables::BOXES.iter().enumerate() {
        let val: u64 = (temp & (mask << (42 - (i * 6)))) >> (42 - (i * 6));
        res = (res << 4) | sbox[tables::BOX_LOOKUP[val as usize]] as u64
    }
    permute(res, 32, &tables::P)
}

fn run_network(block: u64, keys: [u64; 16]) -> u64 {
    let lr: u64 = permute(block, 64, &tables::IP);

    let mut left: u64 = (lr & 0xFF_FF_FF_FF_00_00_00_00) >> 32;
    let mut right: u64 = lr & 0x00_00_00_00_FF_FF_FF_FF;

    for &key in keys.iter() {
        let temp: u64 = 1;
        left = right;
        right = temp ^ f(right, key);
    }
    let switched_sides: u64 = (right << 32) | left;
    permute(switched_sides, 64, &tables::IIP)
}

pub fn encrypt(block: u64, keys: [u64; 16]) -> u64 {
    run_network(block, keys)
}

pub fn decrypt(block: u64, keys: [u64; 16]) -> u64 {
    let mut reverse: [u64; 16] = [0; 16];
    for (i, &key) in keys.iter().rev().enumerate() {
        reverse[i] = key;
    }
    run_network(block, reverse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_test() {
        let sample_inp: u64 = 0x0123456789ABCDEF;
        let sample_key: u64 = 0x133457799BBCDFF1;

        let keys: [u64; 16] = gen_round_keys(sample_key);

        let expected_output: u64 = 0x85E813540F0AB405;

        assert_eq!(encrypt(sample_inp, keys), expected_output);
    }

    #[test]
    fn recurrence_relation_test() {
        // Test from https://people.csail.mit.edu/rivest/pubs/Riv85.txt
        let mut executed_input: u64 = 0x94_74_B8_E8_C7_3B_CA_7D;

        let expected_output: [u64; 16] = [
            0x8D_A7_44_E0_C9_4E_5E_17,
            0x0C_DB_25_E3_BA_3C_6D_79,
            0x47_84_C4_BA_50_06_08_1F,
            0x1C_F1_FC_12_6F_2E_F8_42,
            0xE4_BE_25_00_42_09_8D_13,
            0x7B_FC_5D_C6_AD_B5_79_7C,
            0x1A_B3_B4_D8_20_82_FB_28,
            0xC1_57_6A_14_DE_70_70_97,
            0x73_9B_68_CD_2E_26_78_2A,
            0x2A_59_F0_C4_64_50_6E_DB,
            0xA5_C3_9D_42_51_F0_A8_1E,
            0x72_39_AC_9A_61_07_DD_B1,
            0x07_0C_AC_85_90_24_12_33,
            0x78_F8_7B_6E_3D_FE_CF_61,
            0x95_EC_25_78_C2_C4_33_F0,
            0x1B_1A_2D_DB_4C_64_24_38,
        ];

        for i in 0..16 {
            let keys: [u64; 16] = gen_round_keys(executed_input);
            if i % 2 == 0 {
                executed_input = encrypt(executed_input, keys);
            } else {
                executed_input = decrypt(executed_input, keys);
            }
            assert_eq!(
                executed_input, expected_output[i],
                "X({}) has unexpected val",
                i
            );
        }
    }
}
