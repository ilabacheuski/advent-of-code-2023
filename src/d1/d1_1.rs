pub fn calibrate(input: &str) -> usize {
    input
        .lines()
        // .inspect(|line| println!("line: {line}"))
        .map(|line| {
            let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last = line.chars().rfind(|c| c.is_ascii_digit()).unwrap();
            // println!("first: {first}; last {last}");
            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        // .inspect(|x| println!("about to filter: {x}"))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::d1::input::Input;

    use super::*;

    #[test]
    fn test_d1_1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        assert_eq!(142, calibrate(input));
        assert_eq!(54605, calibrate(Input::new().input1));
    }
}
