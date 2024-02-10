use crate::BmpImage;

pub struct YcbcrImage {
    y_component: Vec<u8>,
    cb_component: Vec<u8>,
    cr_component: Vec<u8>,
    width_px: u32,
}

impl YcbcrImage {
    pub fn get_from_rgb(source: BmpImage) -> YcbcrImage {
    }
}
