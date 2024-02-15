use crate::ShiftedYCR;

#[derive(Debug, PartialEq)]
pub struct MCU {
    values: Vec<Vec<i8>>,
}

#[derive(Debug, PartialEq)]
pub struct ImageAsMCU {
    y_mcu: Vec<MCU>,
    cb_mcu: Vec<MCU>,
    cr_mcu: Vec<MCU>,
}

impl MCU {
    fn new() -> MCU{
        MCU {
            values: vec![vec![0; 8]; 8],
        }
    }

    fn get_mcus(mut input: ShiftedYCR) -> ImageAsMCU {
        let col_mod = 8 - input.width_px % 8;
        let line_mod = 8 - input.height_px % 8;

        if col_mod != 8 {
            for line in input.y_component.iter_mut() {
                for _ in 0..col_mod {
                    line.push(line[line.len() - 1].clone());
                }
            }
            for line in input.cb_component.iter_mut() {
                for _ in 0..(col_mod * input.vertical_sub) {
                    line.push(line[line.len() - 1].clone());
                }
            }
            for line in input.cr_component.iter_mut() {
                for _ in 0..(col_mod * input.vertical_sub) {
                    line.push(line[line.len() - 1].clone());
                }
            }
        }

        if line_mod != 8 {
            for _ in 0..line_mod {
                input.y_component.push(input.y_component[input.y_component.len() - 1].clone());
            }
            for _ in 0..(line_mod * input.horizontal_sub) {
                input.cb_component.push(input.cb_component[input.cb_component.len() - 1].clone());
            }
            for _ in 0..(line_mod * input.horizontal_sub) {
                input.cr_component.push(input.cr_component[input.cr_component.len() - 1].clone());
            }
        }

        let mut y_mcus: Vec<MCU> = Vec::new();
        let mut cb_mcus: Vec<MCU> = Vec::new();
        let mut cr_mcus: Vec<MCU> = Vec::new();

        for i in 0..input.y_component.len()/8 {
            for j in 0..input.y_component[0].len()/8 {
                let mut mcu = MCU::new();
                for x in 0..8 {
                    for y in 0..8 {
                        mcu.values[x][y] = input.y_component[i as usize + x][j as usize + y];
                    }
                }
                y_mcus.push(mcu);
            }
        }

        for i in 0..input.cb_component.len()/8 {
            for j in 0..input.cb_component[i].len()/8 {
                let mut cb_mcu = MCU::new();
                let mut cr_mcu = MCU::new();
                for x in 0..8 {
                    for y in 0..8 {
                        cb_mcu.values[x][y] = input.cb_component[i as usize + x][j as usize + y];
                        cr_mcu.values[x][y] = input.cr_component[i as usize + x][j as usize + y];
                    }
                }
                cb_mcus.push(cb_mcu);
                cr_mcus.push(cr_mcu);
            }
        }

        println!("{}, {}, {}", y_mcus.len(), cb_mcus.len(), cr_mcus.len());

        ImageAsMCU {
            y_mcu: y_mcus,
            cb_mcu: cb_mcus,
            cr_mcu: cr_mcus,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ShiftedYCR, MCU};

    #[test]
    fn subdivision_test() {
        let input = ShiftedYCR {
            y_component: vec![vec![103; 6]; 6], 
            cb_component: vec![vec![-98; 6]; 6],
            cr_component: vec![vec![37; 6]; 6],
            width_px: 6,
            height_px: 6,
            horizontal_sub: 1,
            vertical_sub: 1,
        };

        let output = MCU::get_mcus(input);

        let expected_cb = vec![vec![-98; 8]; 8];

        assert_eq!(output.cb_mcu[0].values, expected_cb);
    }
}
