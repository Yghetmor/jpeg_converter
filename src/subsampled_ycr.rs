use crate::YcbcrImage;

#[derive(Debug, PartialEq)]
pub struct SubsampledYCR {
    y_component: Vec<Vec<u8>>,
    cb_component: Vec<Vec<u8>>,
    cr_component: Vec<Vec<u8>>,
    width_px: u32,
    height_px: u32,
    horizontal_sub: u32,
    vertical_sub: u32,
}

#[derive(Debug, PartialEq)]
pub struct ShiftedYCR {
    pub y_component: Vec<Vec<i8>>,
    pub cb_component: Vec<Vec<i8>>,
    pub cr_component: Vec<Vec<i8>>,
    pub width_px: u32,
    pub height_px: u32,
    pub horizontal_sub: u32,
    pub vertical_sub: u32,
}

impl SubsampledYCR {
    pub fn subsample(input: YcbcrImage) -> SubsampledYCR {
        let mut subs_cb: Vec<Vec<u8>> = Vec::new();
        let mut subs_cr: Vec<Vec<u8>> = Vec::new();

        for (i, line) in input.cb_component.iter().enumerate() {
            if i as u32 % 2 == 0 {
                subs_cb.push(line.iter()
                                .enumerate()
                                .filter(|(i, _)| *i as u32 % 2 == 0)
                                .map(|(_, x)| *x)
                                .collect()
                    )
            }
        }
        for (i, line) in input.cr_component.iter().enumerate() {
            if i as u32 % 2 == 0 {
                subs_cr.push(line.iter()
                                .enumerate()
                                .filter(|(i, _)| *i as u32 % 2 == 0)
                                .map(|(_, x)| *x)
                                .collect()
                    )
            }
        }

        SubsampledYCR {
            y_component: input.y_component,
            cb_component: subs_cb,
            cr_component: subs_cr,
            width_px: input.width_px,
            height_px: input.height_px,
            horizontal_sub: 2,
            vertical_sub: 2,
        }
    }
}

impl ShiftedYCR {
    pub fn shift(input: SubsampledYCR) -> ShiftedYCR {
        let y_shifted = input.y_component.iter()
                                         .map(|line| line.iter()
                                                         .map(|x| ((*x as i32) - 128) as i8)
                                                         .collect()
                                            )
                                         .collect();
        let cb_shifted = input.cb_component.iter()
                                         .map(|line| line.iter()
                                                         .map(|x| ((*x as i32) - 128) as i8)
                                                         .collect()
                                            )
                                         .collect();
        let cr_shifted = input.cr_component.iter()
                                         .map(|line| line.iter()
                                                         .map(|x| ((*x as i32) - 128) as i8)
                                                         .collect()
                                            )
                                         .collect();

        ShiftedYCR {
            y_component: y_shifted,
            cb_component: cb_shifted,
            cr_component: cr_shifted,
            width_px: input.width_px,
            height_px: input.height_px,
            horizontal_sub: input.horizontal_sub,
            vertical_sub: input.vertical_sub,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ShiftedYCR, SubsampledYCR, YcbcrImage};

    #[test]
    fn subsampling_test() {
        let input = YcbcrImage {
            y_component: vec![vec![231, 23], vec![32, 83]],
            cb_component: vec![vec![30, 70], vec![20, 80]],
            cr_component: vec![vec![20, 80], vec![30, 70]],
            width_px: 2,
            height_px: 2,
        };
        let expected = SubsampledYCR {
            y_component: vec![vec![231, 23], vec![32, 83]],
            cb_component: vec![vec![30]],
            cr_component: vec![vec![20]],
            width_px: 2,
            height_px: 2,
            horizontal_sub: 2,
            vertical_sub: 2,
        };

        let output = SubsampledYCR::subsample(input);

        assert_eq!(output, expected);
    }

    #[test]
    fn shift_test() {
        let input = SubsampledYCR {
            y_component: vec![vec![231, 23], vec![32, 83]],
            cb_component: vec![vec![30]],
            cr_component: vec![vec![20]],
            width_px: 2,
            height_px: 2,
            horizontal_sub: 2,
            vertical_sub: 2,
        };
        let expected = ShiftedYCR {
            y_component: vec![vec![103, -105], vec![-96, -45]],
            cb_component: vec![vec![-98]],
            cr_component: vec![vec![-108]],
            width_px: 2,
            height_px: 2,
            horizontal_sub: 2,
            vertical_sub: 2,
        };

        let output = ShiftedYCR::shift(input);

        assert_eq!(output, expected);
    }
}
