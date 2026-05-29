use std::fs;

#[allow(dead_code)]
fn step1(input: String) -> i32 {
    let lines = input.split("\n");
    let mut current_step = 50;
    let mut result = 0;

    for line in lines {
        let direction = line.chars().nth(0).expect("The string is empty");
        
        let step_count: i32 = String::from(&line[1..])
            .parse()
            .expect("End of line is a valid integer");

        let mul = if direction == 'L' { 1 } else {-1 };

        current_step = (current_step + step_count * mul) % 100;


        if current_step == 0 {
            result+=1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_step1_example_should_return_the_right_value() {
        let input = fs::read_to_string("./src/year2025/day01/example.txt")
            .expect("Day 01 example file not found");
        assert_eq!(step1(input), 3);
    }

       #[test]
  fn test_step1_input_should_return_the_right_value() {
        let input = fs::read_to_string("./src/year2025/day01/input.txt")
            .expect("Day 01 input file not found");
        assert_eq!(step1(input), 1152);
    }
}
