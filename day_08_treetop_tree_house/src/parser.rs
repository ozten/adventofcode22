pub fn parse(input: String) -> (usize, usize, Vec<u32>) {
    let mut height = 0;
    let mut width = 0;
    let mut trees: Vec<u32> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        width = line.len();
        height += 1;
        for c in line.chars() {
            let val = c.to_digit(10).unwrap();
            trees.push(val);
        }
    }
    (width, height, trees)
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;

    use crate::parser::parse;

    #[test]
    fn test_parse() -> Result<(), Error> {
        let (actual_width, actual_height, actual_input) =
            parse(read_to_string("src/test_input.txt")?);

        assert_eq!(actual_width, 5);
        assert_eq!(actual_height, 5);
        assert_eq!(*actual_input.get(0).unwrap(), 3);
        assert_eq!(*actual_input.get(24).unwrap(), 0);

        Ok(())
    }

    #[test]
    fn test_parse_big() -> Result<(), Error> {
        let (actual_width, actual_height, actual_input) = parse(read_to_string("src/input.txt")?);

        assert_eq!(actual_width, 99);
        assert_eq!(actual_height, 99);
        assert_eq!(*actual_input.get(0).unwrap(), 1);
        assert_eq!(*actual_input.get(24).unwrap(), 5);

        Ok(())
    }
}
