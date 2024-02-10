use crate::YcbcrImage;

#[derive(Debug, PartialEq)]
pub struct SubsampledYCR {
    y_component: Vec<u8>,
    cb_component: Vec<u8>,
    cr_component: Vec<u8>,
    width_px: u32,
    horizontal_sub: u32,
    vertical_sub: u32,
}

impl SubsampledYCR {
    pub fn subsample(input: YcbcrImage, h_sub: u32, v_sub: u32) -> SubsampledYCR {
        todo!();
    }
}
