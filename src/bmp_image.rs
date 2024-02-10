pub struct BmpImage {
    pub rgb_vector: Vec<u8>,
    pub width_px: u32,
    pub height_px: u32, 
}

impl BmpImage{
    pub fn get_from_file(bmp_path: String) -> BmpImage {
        let mut bmp_vec = std::fs::read(bmp_path).unwrap();
        let width: u32 = bmp_vec[18] as u32 +  bmp_vec[19] as u32 * 256 + bmp_vec[20] as u32 * 65536 + bmp_vec[21] as u32 * 16777216;
        let height: u32 = bmp_vec[22] as u32 + bmp_vec[23] as u32 * 256 + bmp_vec[24] as u32 * 65536 + bmp_vec[25] as u32 * 16777216;

        let offset: u32 = bmp_vec[10] as u32 + bmp_vec[11] as u32 * 256 + bmp_vec[12] as u32 * 65536 + bmp_vec[13] as u32 * 16777216;
        let padding = 4 - (width % 4);
        let mut rgb_vec = Vec::new();
        match padding {
            4 => {
                rgb_vec = bmp_vec.split_off(offset as usize);
            }
            1 => {
                rgb_vec = bmp_vec.split_off(offset as usize)
                                 .iter()
                                 .enumerate()
                                 .filter(|(i, _)| *i as u32 % (width + 1) != 0)
                                 .map(|(_, e)| *e)
                                 .collect();   
            }
            2 => {
                rgb_vec = bmp_vec.split_off(offset as usize)
                                 .iter()
                                 .enumerate()
                                 .filter(|(i, _)| *i as u32 % (width + 1) != 0 && *i as u32 % (width + 2) != 0)
                                 .map(|(_, e)| *e)
                                 .collect();   
            }
            3 => {
                rgb_vec = bmp_vec.split_off(offset as usize)
                                 .iter()
                                 .enumerate()
                                 .filter(|(i, _)| *i as u32 % (width + 1) != 0 && *i as u32 % (width + 2) != 0 && *i as u32 % (width + 3) != 0)
                                 .map(|(_, e)| *e)
                                 .collect();   
            }
            _ => panic!("Error: incorrect padding"),
        }

        BmpImage {
            rgb_vector: rgb_vec,
            width_px: width,
            height_px: height,
        }
    }
}
