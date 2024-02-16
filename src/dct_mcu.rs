use crate::ShiftedYCR;
use std::f32::consts::PI;

#[derive(Debug, PartialEq)]
pub struct MCU {
    values: Vec<Vec<i8>>,
    quantized: bool,
}

#[derive(Debug, PartialEq)]
pub struct ImageAsMCU {
    y_mcu: Vec<MCU>,
    cb_mcu: Vec<MCU>,
    cr_mcu: Vec<MCU>,
    quantized: bool,
}

#[derive(Debug, PartialEq)]
pub struct DctedMcu {
    values: Vec<Vec<f32>>,
}

impl MCU {
    fn new() -> MCU{
        MCU {
            values: vec![vec![0; 8]; 8],
            quantized: false,
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
                        mcu.values[x][y] = input.y_component[(i as usize) * 8 + x][(j as usize) * 8 + y];
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
                        cb_mcu.values[x][y] = input.cb_component[(i as usize) * 8 + x][(j as usize) * 8 + y];
                        cr_mcu.values[x][y] = input.cr_component[(i as usize) * 8 + x][(j as usize) * 8 + y];
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
            quantized: false,
        }
    }
    
    fn calculate_dct(self) -> DctedMcu {
        let mut output_values: Vec<Vec<f32>> = vec![vec![0.0; 8]; 8];

        for u in 0..8 {
            for v in 0..8 {
                let mut res = 0.0;
                for x in 0..8 {
                    for y in 0..8 {
                        res += self.values[x][y] as f32 * (((2.0 * x as f32 + 1.0) * u as f32 * PI) / 16.0).cos() * (((2.0 * y as f32 + 1.0) * v as f32 * PI) / 16.0).cos()
                    }
                }
                if u == 0 && v == 0 {
                    output_values[u][v] = res / 8.0;
                } else if u == 0 || v == 0 {
                    let two: f32 = 2.0;
                    output_values[u][v] = res / (4.0 * two.sqrt());
                } else {
                    output_values[u][v] = res / 4.0;
                }
            }
        }

        DctedMcu { 
            values: output_values
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ShiftedYCR, MCU};

    use super::DctedMcu;

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

    #[test]
    fn dct_test() {
        let input = MCU {
            values: vec![
                vec![-64, -72, -72, -71, -58, -44, -44, -69],
                vec![-62, -64, -93, -92, -41, -83, -107, -70],
                vec![-62, -62, -62, -69, -93, -41, -102, -24],
                vec![-93, -53, -52, -83, -47, -91, -94, -93],
                vec![-83, -32, -3, -21, -97, -113, -21, -38],
                vec![-40, -39, -40, -50, -64, -71, -43, -47],
                vec![-66, -69, -60, -15, 16, -24, -62, -55],
                vec![-21, -7, -39, -107, -93, -64, -63, -63],
            ],
            quantized: false,
        };

        let output = input.calculate_dct();

        let expected = DctedMcu {
            values: vec![
                vec![-477.63, 24.47, 6.93, -25.49, -6.13, -27.83, -0.57, 6.89],
                vec![-65.84, -22.93, -4.66, 15.25, 16.3, -12.69, 12.2, -7.67],
                vec![7.72, -5.29, 14.03, 74.8, 3.88, -15.81, 13.35, -1.86],
                vec![44.54, -25.13, -24.48, -14.24, 3.35, 47.02, -33.93, 13.8],
                vec![-13.63, 22.85, 22.83, -31.1, -53.13, 22.0, -22.31, 20.27],
                vec![11.12, -32.74, -64.88, 40.32, 17.61, -11.14, 11.72, -2.59],
                vec![10.47, 6.93, 62.85, -8.64, -30.16, 17.07, 26.22, -22.7],
                vec![42.47, -31.38, -4.03, -35.84, 0.41, 29.19, 10.36, -27.19]
            ]
        };

        assert_eq!(output, expected);
    }
}
