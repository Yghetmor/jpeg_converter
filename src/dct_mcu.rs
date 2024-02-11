use crate::ShiftedYCR;

pub struct MCU {
    y_component: Vec<Vec<i8>>,
    cb_component: Vec<Vec<i8>>,
    cr_component: Vec<Vec<i8>>,
}

impl MCU {
    fn get_mcu(mut input: ShiftedYCR) -> Vec<MCU> {
        let col_mod = 8 - input.width_px % 8;
        let line_mod = 8 - input.width_px % 8;

        if col_mod != 8 {
            for line in input.y_component.iter_mut() {
                for _ in 0..col_mod {
                    line.push(line[line.len() - 1].clone());
                }
            }
            for line in input.cb_component.iter_mut() {
                for _ in 0..(col_mod / input.vertical_sub) {
                    line.push(line[line.len() - 1].clone());
                }
            }
            for line in input.cr_component.iter_mut() {
                for _ in 0..(col_mod / input.vertical_sub) {
                    line.push(line[line.len() - 1].clone());
                }
            }
        }

        if line_mod != 8 {
            for _ in 0..line_mod {
                input.y_component.push(input.y_component[input.y_component.len() - 1].clone());
            }
            for _ in 0..(line_mod / input.horizontal_sub) {
                input.y_component.push(input.y_component[input.y_component.len() - 1].clone());
            }
            for _ in 0..(line_mod / input.horizontal_sub) {
                input.y_component.push(input.y_component[input.y_component.len() - 1].clone());
            }
        }

        todo!();
    }
}
