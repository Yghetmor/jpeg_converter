use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::BitCode;

pub struct Writer {
    buffer: u32,
    index: u32,
    file: File,
}

impl Writer {
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
            file: the_file,
        }
    }

    pub fn write_bitcode(mut self, bitcode: &BitCode) {
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

    pub fn write_rest(mut self) {
        if self.index != 0 {
            let out: u8 = ((self.buffer as u8) << 8 - self.index) & (u8::MAX >> self.index);
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
