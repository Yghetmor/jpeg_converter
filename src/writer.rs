use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::dct_mcu::ImageAsMCU;
use crate::encoding::{AC_CHROMINANCE_CODES_PER_BITSIZE, AC_CHROMINANCE_VALUES, AC_LUMINANCE_CODES_PER_BITSIZE, AC_LUMINANCE_VALUES, DC_CHROMINANCE_CODES_PER_BITSIZE, DC_CHROMINANCE_VALUES, DC_LUMINANCE_CODES_PER_BITSIZE, DC_LUMINANCE_VALUES};
use crate::BitCode;

pub struct Writer<'a> {
    buffer: u32,
    index: u32,
    file: Box<dyn Write + 'a>,
}

impl<'a> Writer<'a> {
    pub fn new(out_path: &str) -> Writer {
        let path = Path::new(out_path);
        let display = path.display();

        let the_file = match File::create(&path) {
            Err(why) => panic!("couldn't create {} : {}", display, why),
            Ok(file) => file,
        };

        Writer {
            buffer: 0,
            index: 0,
            file: Box::new(the_file),
        }
    }

    pub fn write_bitcode(&mut self, bitcode: &BitCode) {
        if self.index != 0 {
            self.buffer <<= bitcode.num_bits;
        }
        self.buffer |= bitcode.code as u32;
        self.index += bitcode.num_bits;

        while self.index >= 8 {
            let out = self.buffer >> self.index - (self.index / 8) * 8;
            let output: [u8; 1] = [out as u8];
            match self.file.write_all(&output) {
                Err(why) => panic!("couldn't write : {}", why),
                Ok(_) => {
                    //self.buffer &= (u32::MAX >> (32 - self.index + 8));
                    self.index -= 8;
                    self.buffer &= !(u32::MAX << self.index);
                },
            }
            if output[0] == 0xFF {
                self.file.write_all(&[0x00]).unwrap();
            }
        }
    }

    pub fn write_rest(&mut self) {
        if self.index != 0 {
            let out: u8 = ((self.buffer as u8) << 8 - self.index) | (u8::MAX >> self.index);
            let output: [u8; 1] = [out]; 
            match self.file.write_all(&output) {
                Err(why) => panic!("couldn't write rest : {}", why),
                Ok(_) => {
                    self.buffer = 0;
                    self.index = 0;
                }
            }
        }
    }

    pub fn write_quant_table(&mut self, table: &[[f32; 8]; 8], id: u8) {
        self.file.write_all(&[0xFF, 0xDB]).unwrap();
        let length: u8 = 67;
        self.file.write_all(&[0x00, length, id]).unwrap();
        for (i, j) in ZIG_ZAG_ORDER {
            self.file.write_all(&[table[i][j] as u8]).unwrap();
        }
    }

    pub fn write_sof(&mut self, image: &ImageAsMCU, y_quant_table_id: u32, c_quant_table_id: u32) {
        if !image.quantized {
            panic!("image is not quantized : cannot write SOS");
        }
        let mut output: Vec<u8> = vec![0xFF, 0xC0];
        let length: u8 = 17;
        output.push(0x00);
        output.push(length);
        output.push(8); //Precision
        output.push((image.height_px >> 8) as u8);
        output.push(image.height_px as u8);
        output.push((image.width_px >> 8) as u8);
        output.push(image.width_px as u8);
        output.push(3); //nb of components
        for i in 1..4 {
            output.push(i); //component id
            output.push(0x11); //horizontal and vertical sampling factor
            if i == 1 {
                output.push(y_quant_table_id as u8);
            } else {
                output.push(c_quant_table_id as u8);
            }
        }
        self.file.write_all(&output).unwrap();
        self.write_huffman_table(&DC_LUMINANCE_CODES_PER_BITSIZE, DC_LUMINANCE_VALUES.to_vec(), 0, 0);
        self.write_huffman_table(&DC_CHROMINANCE_CODES_PER_BITSIZE, DC_CHROMINANCE_VALUES.to_vec(), 0, 1);
        self.write_huffman_table(&AC_LUMINANCE_CODES_PER_BITSIZE, AC_LUMINANCE_VALUES.to_vec(), 1, 2);
        self.write_huffman_table(&AC_CHROMINANCE_CODES_PER_BITSIZE, AC_CHROMINANCE_VALUES.to_vec(), 1, 3);
    }

    pub fn write_sos(&mut self) {
        self.file.write_all(&[0xFF, 0xDA, 0x00, 12, 3]).unwrap();
        self.file.write_all(&[0x01, 0x02]).unwrap();
        self.file.write_all(&[0x02, 0x13]).unwrap();
        self.file.write_all(&[0x03, 0x13]).unwrap();
        self.file.write_all(&[0x00, 0x3F, 0x00]).unwrap();
    }

    fn write_huffman_table(&mut self, codes: &[u8; 16], vals: Vec<u8>, tc: u8, th: u8) {
        self.file.write_all(&[0xFF, 0xC4]).unwrap();
        let length: u8 = 2 + 1 + 16 + (vals.len() as u8);
        self.file.write_all(&[0x00, length]).unwrap();
        let tables_info: u8 = (tc << 4) | th;
        self.file.write_all(&[tables_info]).unwrap();
        self.file.write_all(codes).unwrap();
        self.file.write_all(&vals).unwrap();
    }

    pub fn write_soi(&mut self) {
        self.file.write_all(&[0xFF, 0xD8]).unwrap();
    }

    pub fn write_eoi(&mut self) {
        self.file.write_all(&[0xFF, 0xD9]).unwrap();
    }
}

const ZIG_ZAG_ORDER: [(usize, usize); 64] = [(0,0), (0,1), (1,0), (2,0), (1,1), (0,2), (0,3), (1,2), (2,1), (3,0), (4,0), (3,1), (2,2), (1,3), (0,4), (0,5), (1,4), (2,3), (3,2), (4,1), (5,0), (6,0), (5,1), (4,2), (3,3), (2,4), (1,5), (0,6), (0,7), (1,6), (2,5), (3,4), (4,3), (5,2), (6,1), (7,0), (7,1), (6,2), (5,3), (4,4), (3,5), (2,6), (1,7), (2,7), (3,6), (4,5), (5,4), (6,3), (7,2), (7,3), (6,4), (5,5), (4,6), (3,7), (4,7), (5,6), (6,5), (7,4), (7,5), (6,6), (5,7), (6,7), (7,6), (7,7)];

#[cfg(test)]
mod tests {
    use crate::encoding::{DC_LUMINANCE_CODES_PER_BITSIZE, DC_LUMINANCE_VALUES};
    use crate::{BitCode, Writer};
    use crate::dct_mcu::Y_QUANTIZATION_TABLE;

    #[test]
    fn write_bitcode_test() {
        let mut output: Vec<u8> = Vec::new();
        let mut writer = Writer {
            buffer: 0b011011,
            index: 6,
            file: Box::new(&mut output),
        };

        writer.write_bitcode(&BitCode {
            code: 0b01110101,
            num_bits: 8,
        });

        let expected: Vec<u8> = [0b01101101].to_vec();
        assert_eq!(writer.buffer, 0b110101);
        assert_eq!(writer.index, 6);
        drop(writer);
        assert_eq!(output, expected);
    }

    #[test]
    fn write_rest_test() {
        let mut output: Vec<u8> = Vec::new();
        let mut writer = Writer {
            buffer: 0b010,
            index: 3,
            file: Box::new(&mut output),
        };

        writer.write_rest();

        let expected: Vec<u8> = [0b01011111].to_vec();
        assert_eq!(writer.buffer, 0);
        assert_eq!(writer.index, 0);
        drop(writer);
        assert_eq!(output, expected);
    }

    #[test]
    fn write_quant_table_test() {
        let mut output: Vec<u8> = Vec::new();
        let mut writer = Writer {
            buffer: 0,
            index: 0,
            file: Box::new(&mut output),
        };

        writer.write_quant_table(&Y_QUANTIZATION_TABLE, 0);

        let expected: Vec<u8> = vec![
            0xFF, 0xDB, 0x00, 67, 0, 16, 11, 12, 14, 12, 10, 16, 14, 13, 14, 18, 17, 16, 19, 24, 40, 26, 24, 22, 22, 24, 49, 35, 37, 29, 40, 58, 51, 61, 60, 57, 51, 56, 55, 64, 72, 92, 78, 64, 68, 87, 69, 55, 56, 80, 109, 81, 87, 95, 98, 103, 104, 103, 62, 77, 113, 121, 112, 100, 120, 92, 101, 103, 99 
        ];

        drop(writer);
        assert_eq!(output, expected);
    }

    #[test]
    fn write_huffman_table_test() {
        let mut output: Vec<u8> = Vec::new();
        let mut writer = Writer {
            buffer: 0,
            index: 0,
            file: Box::new(&mut output),
        };

        writer.write_huffman_table(&DC_LUMINANCE_CODES_PER_BITSIZE, DC_LUMINANCE_VALUES.to_vec(), 0, 0);

        let expected: Vec<u8> = vec![
            0xFF, 0xC4, 00, 31, 0, 0,1,5,1,1,1,1,1,1,0,0,0,0,0,0,0, 0,1,2,3,4,5,6,7,8,9,10,11 
        ];

        drop(writer);
        assert_eq!(output, expected);
    }
}
