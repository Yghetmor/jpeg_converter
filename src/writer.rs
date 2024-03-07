use std::fs::File;
use std::io::Write;
use std::path::Path;

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
                    self.buffer &= u32::MAX >> (32 - self.index + 8);
                    self.index -= 8;
                },
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
}

#[cfg(test)]
mod tests {
    use crate::{BitCode, Writer};

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
}
