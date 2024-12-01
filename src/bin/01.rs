advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut col1, mut col2) = read_input(input);
    col2.sort();
    col1.sort();
    let mut result = 0;
    for i in 0..col1.len() {
        let diff = col2[i] as i32 - col1[i] as i32;
        result += diff.abs() as u32;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (col1, col2) = read_input(input);
    let occurrence_count = col2
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
    let result_sum = col1
        .iter()
        .map(|x| {
            if let Some(count) = occurrence_count.get(&(x)) {
                return x * count;
            } else {
                return 0;
            }
        })
        .sum::<u32>();
    Some(result_sum)
}

fn read_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let rows = input.split("\n");
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for row in rows {
        let splitted_row: Vec<&str> = row.split_whitespace().collect();

        if let Ok(num1) = splitted_row[0].parse::<u32>() {
            col1.push(num1);
        }
        if let Ok(num2) = splitted_row[1].parse::<u32>() {
            col2.push(num2);
        }
    }
    (col1, col2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
