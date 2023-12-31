pub fn gear_ratios1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut result = 0;
    let mut deb = Vec::new();

    for (i, &line) in lines.iter().enumerate() {
        let mut line_iter = line.iter().enumerate();
        while let Some((j, &char)) = line_iter.next() {
            if let b'0'..=b'9' = char {
                let start_idx = j;
                let mut end_idx = j;
                let mut number: usize = (char - b'0') as usize;

                let mut next_char = line_iter.next();
                while let Some((idx, char)) = next_char {
                    if char.is_ascii_digit() {
                        number = number * 10 + (char - b'0') as usize;
                    } else {
                        end_idx = idx;
                        break;
                    }
                    next_char = line_iter.next();
                }

                if next_char.is_none() {
                    end_idx = line.len() - 1;
                }

                let mut is_part = false;

                // at right end
                if let Some((_, &char)) = next_char {
                    if char != b'.' {
                        is_part = true;
                    }
                }

                // at left end
                if start_idx > 0 {
                    if let Some(&char) = line.get(start_idx - 1) {
                        if char != b'.' {
                            is_part = true;
                        }
                    }
                }

                let left_bound = if start_idx == 0 { 0 } else { start_idx - 1 };
                let right_bound = if end_idx == line.len() - 1 {
                    line.len() - 1
                } else {
                    end_idx + 1
                };
                // up
                if i > 0 {
                    if let Some(&test_line) = lines.get(i - 1) {
                        let res = check_slice(&test_line[left_bound..right_bound]);
                        if res {
                            is_part = true;
                        }
                    }
                }

                // bottom
                if i != lines.len() - 1 {
                    if let Some(&test_line) = lines.get(i + 1) {
                        let res = check_slice(&test_line[left_bound..right_bound]);
                        if res {
                            is_part = true;
                        }
                    }
                }

                if is_part {
                    deb.push(number);
                    result += number;
                }
            }
        }
    }

    result
}

fn check_slice(slice: &[u8]) -> bool {
    slice
        .iter()
        .any(|&char| !(char == b'.' || char.is_ascii_digit()))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    pub fn load(path: &str) -> String {
        fs::read_to_string(path).expect("Something went wrong reading the file")
    }

    #[test]
    fn test_gear_ratios1() {
        assert_eq!(4361, gear_ratios1(load("src/d3/test1.txt").as_str()));
    }

    #[test]
    fn test_gear_ratios2() {
        assert_eq!(546312, gear_ratios1(load("src/d3/test2.txt").as_str()));
    }
}
