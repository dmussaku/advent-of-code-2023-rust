use std::fs;


pub fn read_input(path: &str) -> Vec<Vec<char>>{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    contents.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn filter_input_to_have_only_digits(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // filter input to only include digits
    input
        .iter()
        .map(|line| line
            .iter()
            .filter(|c| c.is_digit(10))
            .map(|c| *c)
            .collect()
        )
        .collect()    
}

pub fn run_part_1(input: Vec<Vec<char>>) -> usize {
    let filtered_input = filter_input_to_have_only_digits(input);
    let mut sum = 0;
    for array in filtered_input {
        let first_elem = array.first().unwrap();
        let last_elem = array.last().unwrap();
        let num = format!("{}{}", first_elem, last_elem).parse::<usize>().unwrap();
        sum += num;
    }
    sum
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input() {
        let result = read_input("src/days/day1/input_files/test_input.txt");
        assert_eq!(result, 
            vec![vec!['1', 'a', 'b', 'c', '2'], 
            vec!['p', 'q', 'r', '3', 's', 't', 'u', '8', 'v', 'w', 'x'], 
            vec!['a', '1', 'b', '2', 'c', '3', 'd', '4', 'e', '5', 'f'], 
            vec!['t', 'r', 'e', 'b', '7', 'u', 'c', 'h', 'e', 't']]
        );
    }

    #[test]
    fn test_filter_input_to_have_only_digits() {
        let input = vec![vec!['1', 'a', 'b', 'c', '2'], 
            vec!['p', 'q', 'r', '3', 's', 't', 'u', '8', 'v', 'w', 'x'], 
            vec!['a', '1', 'b', '2', 'c', '3', 'd', '4', 'e', '5', 'f'], 
            vec!['t', 'r', 'e', 'b', '7', 'u', 'c', 'h', 'e', 't']];
        let result = filter_input_to_have_only_digits(input);
        assert_eq!(result, 
            vec![vec!['1', '2'], 
            vec!['3', '8'], 
            vec!['1', '2', '3', '4', '5'], 
            vec!['7']]
        );
    }

    #[test]
    fn test_run_part_1() {
        let input = vec![vec!['1', 'a', 'b', 'c', '2'], 
            vec!['p', 'q', 'r', '3', 's', 't', 'u', '8', 'v', 'w', 'x'], 
            vec!['a', '1', 'b', '2', 'c', '3', 'd', '4', 'e', '5', 'f'], 
            vec!['t', 'r', 'e', 'b', '7', 'u', 'c', 'h', 'e', 't']];
        let result = run_part_1(input);
        assert_eq!(result, 142);
    }
}

