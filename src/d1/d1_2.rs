pub fn calibrate2(input: &str) -> usize {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        // .inspect(|line| println!("line: {line}"))
        .map(|line| {
            let mut left: Option<usize> = None;
            let mut left_idx: Option<usize> = None;

            let mut right: Option<usize> = None;
            let mut right_idx: Option<usize> = None;

            for (n, &word) in numbers.iter().enumerate() {
                let number_idx = line.find(n.to_string().as_str());
                let word_idx = line.find(word);

                // find min indeces
                let min_idx = min_idx(number_idx, word_idx);

                if let Some(min) = min_idx {
                    if let Some(lef_min_idx) = left_idx {
                        if min < lef_min_idx {
                            left_idx = Some(min);
                            left = Some(n);
                        }
                    } else {
                        left_idx = Some(min);
                        left = Some(n);
                    }
                }

                let number_idx = line.rfind(n.to_string().as_str());
                let word_idx = line.rfind(word);

                let max_idx = max_idx(number_idx, word_idx);

                if let Some(max) = max_idx {
                    if let Some(right_max_idx) = right_idx {
                        if max > right_max_idx {
                            right_idx = Some(max);
                            right = Some(n);
                        }
                    } else {
                        right_idx = Some(max);
                        right = Some(n);
                    }
                }
            }

            left.unwrap() * 10 + right.unwrap()
        })
        // .inspect(|x| println!("about to filter: {x}"))
        .sum()
}

fn min_idx(idx1: Option<usize>, idx2: Option<usize>) -> Option<usize> {
    match (idx1, idx2) {
        (Some(idx1), Some(idx2)) => {
            if idx1 < idx2 {
                Some(idx1)
            } else {
                Some(idx2)
            }
        }
        (Some(idx1), None) => Some(idx1),
        (None, Some(idx2)) => Some(idx2),
        (None, None) => None,
    }
}

fn max_idx(idx1: Option<usize>, idx2: Option<usize>) -> Option<usize> {
    match (idx1, idx2) {
        (Some(idx1), Some(idx2)) => {
            if idx1 > idx2 {
                Some(idx1)
            } else {
                Some(idx2)
            }
        }
        (Some(idx1), None) => Some(idx1),
        (None, Some(idx2)) => Some(idx2),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::d1::input::Input;

    use super::*;

    #[test]
    fn test_d1_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(50, calibrate2("5one1twozerofive9zero"));
        assert_eq!(142, calibrate2(input));

        assert_eq!(55429, calibrate2(Input::new().input1));
    }
}
