#[derive(Debug)]
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
        let padding = 4 - (3 * width % 4);
        //let mut rgb_vec = bmp_vec.split_off((offset - 1) as usize);
        let bmp_vec = bmp_vec.split_off(offset as usize);
        let mut rgb_vec: Vec<u8> = Vec::new(); 
        match padding {
            4 => {
                rgb_vec = bmp_vec;
            }
            1 => {
                let mut cnt = 1;
                for val in bmp_vec {
                    if cnt <= 3 * width {
                        rgb_vec.push(val);
                    }
                    if cnt == 3 * width + 1 {
                        cnt = 0;
                    }
                    cnt += 1;
                }
                /*
                rgb_vec = rgb_vec.iter()
                                 .enumerate()
                                 .filter(|(i, _)| *i as u32 % (3 * width + 1) != 0)
                                 .map(|(_, e)| *e)
                                 .collect();
*/
            }
            2 => {
                let mut cnt = 1;
                for val in bmp_vec {
                    if cnt <= 3 * width {
                        rgb_vec.push(val);
                    }
                    if cnt == 3 * width + 2 {
                        cnt = 0;
                    }
                    cnt += 1;
                }
                /*
                rgb_vec = rgb_vec.iter()
                                 .enumerate()
                                 .filter(|(i, _)| *i as u32 % (3 * width) != 0 && *i as u32 % (3 * width + 1) != 0)
                                 .map(|(_, e)| *e)
                                 .collect();   
*/
            }
            3 => {
                let mut cnt = 1;
                for val in bmp_vec {
                    if cnt <= 3 * width {
                        rgb_vec.push(val);
                    }
                    if cnt == 3 * width + 3 {
                        cnt = 0;
                    }
                    cnt += 1;
                }
                /*
                rgb_vec = rgb_vec.iter()
                                 .enumerate()
                                 .filter(|(i, _)| *i as u32 % (3 * width + 1) != 0 && *i as u32 % (3 * width + 2) != 0 && *i as u32 % (3 * width + 3) != 0)
                                 .map(|(_, e)| *e)
                                 .collect();   
*/
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

#[cfg(test)]
mod tests {
    use crate::BmpImage;

    #[test]
    fn get_from_file_test() {
        let path1 = String::from("testing/2px.bmp");
        let path2 = String::from("testing/3px.bmp");

        let image1 = BmpImage::get_from_file(path1);
        let image2 = BmpImage::get_from_file(path2);

        let expected1: Vec<u8> = vec![0, 255, 0, 255, 0, 0, 255, 255, 255, 0, 0, 255];
        let expected2: Vec<u8> = vec![255, 0, 0, 255, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 255, 0, 255, 0, 0, 0, 255, 0, 255, 0, 0, 255, 0];

        assert_eq!(expected1, image1.rgb_vector);
        assert_eq!(expected2, image2.rgb_vector);
    }
}
