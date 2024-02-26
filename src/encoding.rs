#[derive(Debug, PartialEq)]
pub struct RLEncoded {
    dc_coef: i8,
    ac_coefs: Vec<(u8, i8)>
}

impl RLEncoded {
    pub fn new(input: Vec<i8>) -> RLEncoded {
        let mut cnt: u8 = 0;
        let mut ac_coef: Vec<(u8, i8)> = Vec::new();
        for i in 1..64 {
            if input[i] == 0 {
                cnt += 1;
                if i == 63 {
                    ac_coef.push((0, 0));
                }
            } else {
                while cnt >= 15 {
                    ac_coef.push((15, 0));
                    cnt -= 15;
                }
                ac_coef.push((cnt, input[i]));
                cnt = 0;
            }
        }

        RLEncoded {
            dc_coef: input[0],
            ac_coefs: ac_coef,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RLEncoded;

    #[test]
    fn rl_encode_test() {
        let input1: Vec<i8> = vec![-30, 2, -5, 0, -2, 1, -2, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; 
        let output1 = RLEncoded::new(input1);

        let input2: Vec<i8> = vec![-30, 2, -5, 0, -2, 1, -2, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; 
        let output2 = RLEncoded::new(input2);

        let expected1 = RLEncoded {
            dc_coef: -30,
            ac_coefs: vec![(0, 2), (0, -5), (1, -2), (0, 1), (0, -2), (2, 1), (3, 1), (3, 1), (0, 0)],
        };

        let expected2 = RLEncoded {
            dc_coef: -30,
            ac_coefs: vec![(0, 2), (0, -5), (1, -2), (0, 1), (0, -2), (2, 1), (3, 1), (3, 1), (15, 0), (15, 0), (15, 0), (0, 1)],
        };

        assert_eq!(output1, expected1);
        assert_eq!(output2, expected2);
    }
}
