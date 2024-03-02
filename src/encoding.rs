const DC_LUMINANCE_CODES_PER_BITSIZE: [u8; 16] = [0,1,5,1,1,1,1,1,1,0,0,0,0,0,0,0];
const DC_LUMINANCE_VALUES: [u8; 12] = [0,1,2,3,4,5,6,7,8,9,10,11];
const AC_LUMINANCE_CODES_PER_BITSIZE: [u8; 16] = [0,2,1,3,3,2,4,3,5,5,4,4,0,0,1,125];
const AC_LUMINANCE_VALUES: [u8; 162] = [0x01,0x02,0x03,0x00,0x04,0x11,0x05,0x12,0x21,0x31,0x41,0x06,0x13,0x51,0x61,0x07,0x22,0x71,0x14,0x32,0x81,0x91,0xA1,0x08,0x23,0x42,0xB1,0xC1,0x15,0x52,0xD1,0xF0,0x24,0x33,0x62,0x72,0x82,0x09,0x0A,0x16,0x17,0x18,0x19,0x1A,0x25,0x26,0x27,0x28,0x29,0x2A,0x34,0x35,0x36,0x37,0x38,0x39,0x3A,0x43,0x44,0x45,0x46,0x47,0x48,0x49,0x4A,0x53,0x54,0x55,0x56,0x57,0x58,0x59,0x5A,0x63,0x64,0x65,0x66,0x67,0x68,0x69,0x6A,0x73,0x74,0x75,0x76,0x77,0x78,0x79,0x7A,0x83,0x84,0x85,0x86,0x87,0x88,0x89,0x8A,0x92,0x93,0x94,0x95,0x96,0x97,0x98,0x99,0x9A,0xA2,0xA3,0xA4,0xA5,0xA6,0xA7,0xA8,0xA9,0xAA,0xB2,0xB3,0xB4,0xB5,0xB6,0xB7,0xB8,0xB9,0xBA,0xC2,0xC3,0xC4,0xC5,0xC6,0xC7,0xC8,0xC9,0xCA,0xD2,0xD3,0xD4,0xD5,0xD6,0xD7,0xD8,0xD9,0xDA,0xE1,0xE2,0xE3,0xE4,0xE5,0xE6,0xE7,0xE8,0xE9,0xEA,0xF1,0xF2,0xF3,0xF4,0xF5,0xF6,0xF7,0xF8,0xF9,0xFA];
const DC_CHROMINANCE_CODES_PER_BITSIZE: [u8; 16] = [0,3,1,1,1,1,1,1,1,1,1,0,0,0,0,0];
const DC_CHROMINANCE_VALUES: [u8; 12] = [0,1,2,3,4,5,6,7,8,9,10,11];
const AC_CHROMINANCE_CODES_PER_BITSIZE: [u8; 16] = [0,2,1,2,4,4,3,4,7,5,4,4,0,1,2,119];
const AC_CHROMINANCE_VALUES: [u8; 162] = [0x00,0x01,0x02,0x03,0x11,0x04,0x05,0x21,0x31,0x06,0x12,0x41,0x51,0x07,0x61,0x71,0x13,0x22,0x32,0x81,0x08,0x14,0x42,0x91,0xA1,0xB1,0xC1,0x09,0x23,0x33,0x52,0xF0,0x15,0x62,0x72,0xD1,0x0A,0x16,0x24,0x34,0xE1,0x25,0xF1,0x17,0x18,0x19,0x1A,0x26,0x27,0x28,0x29,0x2A,0x35,0x36,0x37,0x38,0x39,0x3A,0x43,0x44,0x45,0x46,0x47,0x48,0x49,0x4A,0x53,0x54,0x55,0x56,0x57,0x58,0x59,0x5A,0x63,0x64,0x65,0x66,0x67,0x68,0x69,0x6A,0x73,0x74,0x75,0x76,0x77,0x78,0x79,0x7A,0x82,0x83,0x84,0x85,0x86,0x87,0x88,0x89,0x8A,0x92,0x93,0x94,0x95,0x96,0x97,0x98,0x99,0x9A,0xA2,0xA3,0xA4,0xA5,0xA6,0xA7,0xA8,0xA9,0xAA,0xB2,0xB3,0xB4,0xB5,0xB6,0xB7,0xB8,0xB9,0xBA,0xC2,0xC3,0xC4,0xC5,0xC6,0xC7,0xC8,0xC9,0xCA,0xD2,0xD3,0xD4,0xD5,0xD6,0xD7,0xD8,0xD9,0xDA,0xE2,0xE3,0xE4,0xE5,0xE6,0xE7,0xE8,0xE9,0xEA,0xF2,0xF3,0xF4,0xF5,0xF6,0xF7,0xF8,0xF9,0xFA];

#[derive(Debug, PartialEq, Clone)]
pub struct BitCode {
    code: u16,
    num_bits: u32,
}

impl BitCode {
    fn new() -> BitCode {
        BitCode {
            code: 0,
            num_bits: 0,
        }
    }

    fn new_with_params(the_code: u16, the_num_bits: u32) -> BitCode {
        BitCode {
            code: the_code,
            num_bits: the_num_bits,
        }
    }

    pub fn get_bitcode(input: i8, codes: &Vec<BitCode>) -> BitCode {
        let mut negative = false;
        if input < 0 {
            negative = true;
        }
        let input = input.abs();
        let mut length = 0;
        while 2_i32.pow(length) <= (input as i32) {
            length += 1;
        }
        let the_code = codes[length as usize].clone();
        let bit_representation: u16 = if negative {(((input * (-1)) as i16) + (2_i16.pow(length) - 1)).try_into().unwrap()} else {input as u16};
        BitCode {
            code: (the_code.code << length) | bit_representation,
            num_bits: length + the_code.num_bits,
        }
    }

    pub fn calculate_huffman_codes(codes_per_bitsize: Vec<u8>, values: Vec<u8>) -> Vec<BitCode> {
        let mut values_index = 0;
        let mut huffman_code = 0;
        let mut output = vec![BitCode::new(); 256];
        for num_bits in 1..=codes_per_bitsize.len() {
            for _ in 0..codes_per_bitsize[num_bits - 1] {
                output[values[values_index] as usize] = BitCode {
                    code: huffman_code,
                    num_bits: num_bits as u32,
                };
                huffman_code += 1;
                values_index += 1;
            }
            huffman_code <<= 1;
        }

        output
    }
}

#[derive(Debug, PartialEq)]
pub struct RLEncoded {
    dc_coef: i8,
    ac_coefs: Vec<(u8, i8)>
}

impl RLEncoded {
    pub fn new(input: Vec<i8>) -> RLEncoded {
        let mut cnt: u8 = 0;
        let mut ac_coef: Vec<(u8, i8)> = Vec::new();
        for i in 1..64 {
            if input[i] == 0 {
                cnt += 1;
                if i == 63 {
                    ac_coef.push((0, 0));
                }
            } else {
                while cnt >= 16 {
                    ac_coef.push((15, 0));
                    cnt -= 16;
                }
                ac_coef.push((cnt, input[i]));
                cnt = 0;
            }
        }

        RLEncoded {
            dc_coef: input[0],
            ac_coefs: ac_coef,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::encoding::AC_LUMINANCE_CODES_PER_BITSIZE;
    use crate::encoding::AC_LUMINANCE_VALUES;
    use crate::RLEncoded;
    use crate::BitCode;

    use super::DC_LUMINANCE_CODES_PER_BITSIZE;
    use super::DC_LUMINANCE_VALUES;

    #[test]
    fn calculate_huffman_codes_test() {
        let codes_input = [0, 2, 1, 3];
        let values_input = [0x01,0x02,0x03,0x00,0x04,0x11];
        let output = BitCode::calculate_huffman_codes(codes_input.to_vec(), values_input.to_vec());

        let mut expected = vec![BitCode::new(); 256];
        expected[1] = BitCode::new_with_params(0, 2);
        expected[2] = BitCode::new_with_params(1, 2);
        expected[3] = BitCode::new_with_params(4, 3);
        expected[0] = BitCode::new_with_params(10, 4);
        expected[4] = BitCode::new_with_params(11, 4);
        expected[0x11] = BitCode::new_with_params(12, 4);

        assert_eq!(output, expected);
    }

    #[test]
    fn get_bitcode_test() {
        let dc_lum_codes = BitCode::calculate_huffman_codes(DC_LUMINANCE_CODES_PER_BITSIZE.to_vec(), DC_LUMINANCE_VALUES.to_vec());
        let ac_chrom_codes = BitCode::calculate_huffman_codes(AC_LUMINANCE_CODES_PER_BITSIZE.to_vec(), AC_LUMINANCE_VALUES.to_vec());
        let input1: i8 = -30;
        let input2: i8 = 3;
        let input3: i8 = 10;
        let output1 = BitCode::get_bitcode(input1, &dc_lum_codes);
        let output2 = BitCode::get_bitcode(input2, &dc_lum_codes);
        let output3 = BitCode::get_bitcode(input3, &ac_chrom_codes);

        let expected1 = BitCode::new_with_params(0b11000001, 8);
        let expected2 = BitCode::new_with_params(0b01111, 5);
        let expected3 = BitCode::new_with_params(0b10111010, 8);

        assert_eq!(output1, expected1);
        assert_eq!(output2, expected2);
        assert_eq!(output3, expected3);
    }

    #[test]
    fn rl_encode_test() {
        let input1: Vec<i8> = vec![-30, 2, -5, 0, -2, 1, -2, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; 
        let output1 = RLEncoded::new(input1);

        let input2: Vec<i8> = vec![-30, 2, -5, 0, -2, 1, -2, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; 
        let output2 = RLEncoded::new(input2);

        let expected1 = RLEncoded {
            dc_coef: -30,
            ac_coefs: vec![(0, 2), (0, -5), (1, -2), (0, 1), (0, -2), (2, 1), (3, 1), (3, 1), (0, 0)],
        };

        let expected2 = RLEncoded {
            dc_coef: -30,
            ac_coefs: vec![(0, 2), (0, -5), (1, -2), (0, 1), (0, -2), (2, 1), (3, 1), (3, 1), (15, 0), (15, 0), (13, 1)],
        };

        assert_eq!(output1, expected1);
        assert_eq!(output2, expected2);
    }
}
