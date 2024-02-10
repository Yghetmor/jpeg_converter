use crate::BmpImage;

#[derive(Debug, PartialEq)]
pub struct YcbcrImage {
    y_component: Vec<Vec<u8>>,
    cb_component: Vec<Vec<u8>>,
    cr_component: Vec<Vec<u8>>,
    width_px: u32,
}

impl YcbcrImage {
    pub fn get_from_rgb(source: BmpImage) -> YcbcrImage {
        let mut y_vec = Vec::new();
        let mut cb_vec = Vec::new();
        let mut cr_vec = Vec::new();
        let mut i = 0;
        while i < source.rgb_vector.len() {
            let mut y_line_vec = Vec::new();
            let mut cb_line_vec = Vec::new();
            let mut cr_line_vec = Vec::new();
            for _ in 0..source.width_px {
            //while i as u32 % (source.width_px * 3) != 0 {
                y_line_vec.push((0.299 * (source.rgb_vector[i + 2] as f32) + 0.587 * (source.rgb_vector[i + 1] as f32) + 0.114 * (source.rgb_vector[i] as f32)) as u8);
                cb_line_vec.push((-0.1687 * (source.rgb_vector[i + 2] as f32) - 0.3313 * (source.rgb_vector[i + 1] as f32) + 0.5 * (source.rgb_vector[i] as f32) + 128 as f32) as u8);
                cr_line_vec.push((0.5 * (source.rgb_vector[i + 2] as f32) - 0.4187 * (source.rgb_vector[i + 1] as f32) - 0.0813 * (source.rgb_vector[i] as f32) + 128 as f32) as u8);
                i += 3;
            }
            y_vec.push(y_line_vec);
            cb_vec.push(cb_line_vec);
            cr_vec.push(cr_line_vec);
        }

        YcbcrImage {
            y_component: y_vec,
            cb_component: cb_vec,
            cr_component: cr_vec,
            width_px: source.width_px,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{BmpImage, YcbcrImage};

    #[test]
    fn conversion_test() {
        let input = BmpImage {
            rgb_vector: vec![100, 150, 200, 50, 200, 100],
            width_px: 2,
            height_px: 1,
        };
        let expected = YcbcrImage {
            y_component: vec![vec![159, 153]],
            cb_component: vec![vec![94, 69]],
            cr_component: vec![vec![157, 90]],
            width_px: 2,
        };

        let output = YcbcrImage::get_from_rgb(input);

        assert_eq!(output, expected);
    }
}
