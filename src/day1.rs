use crate::day::Day;
use std::fmt::Display;

pub struct Day1 {}

impl Day for Day1 {
    fn part1(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut sum_calibration = 0;
        for line in input.lines() {
            sum_calibration += get_digits_only_calibration_value_for_line(line);
        }
        return Ok(Box::new(sum_calibration));
    }

    fn part2(&self, input: &str) -> Result<Box<dyn Display>, &str> {
        let mut sum_calibration = 0;
        for line in input.lines() {
            sum_calibration += get_calibration_value_for_line(line);
        }
        return Ok(Box::new(sum_calibration));
    }
}

fn get_digits_only_calibration_value_for_line(line: &str) -> i32 {
    let mut calibration_value: i32 = 0;

    // Get first digit
    let mut chars = line.chars();
    while let Some(char) = chars.next() {
        if let Some(n) = char.to_digit(10) {
            calibration_value += 10 * n as i32;
            break;
        }
    }

    // Get last digit
    let mut chars = line.chars();
    while let Some(char) = chars.next_back() {
        if let Some(n) = char.to_digit(10) {
            calibration_value += n as i32;
            break;
        }
    }

    return calibration_value;
}

// Returns the digit that matches a given fragment, or None if it doesn't match.
fn match_number(fragment: &str) -> Option<i32> {
    match fragment {
        "one" => return Some(1),
        "two" => return Some(2),
        "three" => return Some(3),
        "four" => return Some(4),
        "five" => return Some(5),
        "six" => return Some(6),
        "seven" => return Some(7),
        "eight" => return Some(8),
        "nine" => return Some(9),
        _ => return None,
    }
}

fn is_valid_prefix(fragment: &str) -> bool {
    return fragment.is_empty()
        || "one".starts_with(fragment)
        || "two".starts_with(fragment)
        || "three".starts_with(fragment)
        || "four".starts_with(fragment)
        || "five".starts_with(fragment)
        || "six".starts_with(fragment)
        || "seven".starts_with(fragment)
        || "eight".starts_with(fragment)
        || "nine".starts_with(fragment);
}

fn is_valid_suffix(fragment: &str) -> bool {
    return fragment.is_empty()
        || "one".ends_with(fragment)
        || "two".ends_with(fragment)
        || "three".ends_with(fragment)
        || "four".ends_with(fragment)
        || "five".ends_with(fragment)
        || "six".ends_with(fragment)
        || "seven".ends_with(fragment)
        || "eight".ends_with(fragment)
        || "nine".ends_with(fragment);
}

fn get_calibration_value_for_line(line: &str) -> i32 {
    let mut calibration_value: i32 = 0;

    // Get first digit respecting matching
    let mut chars = line.chars();
    let mut fragment = String::new();
    while let Some(char) = chars.next() {
        // Validate single numeric digit
        if let Some(n) = char.to_digit(10) {
            calibration_value += 10 * n as i32;
            break;
        }

        // Validate string fragment
        fragment.push(char);
        if let Some(n) = match_number(fragment.as_str()) {
            calibration_value += 10 * n as i32;
            break;
        }
        while !fragment.is_empty() && !is_valid_prefix(fragment.as_str()) {
            fragment.remove(0);
        }
    }

    // Get last digit
    let mut chars = line.chars();
    while let Some(char) = chars.next_back() {
        // Validate single numeric digit
        if let Some(n) = char.to_digit(10) {
            calibration_value += n as i32;
            break;
        }

        // Validate string fragment
        fragment.insert(0, char);
        if let Some(n) = match_number(fragment.as_str()) {
            calibration_value += n as i32;
            break;
        }
        while !fragment.is_empty() && !is_valid_suffix(fragment.as_str()) {
            fragment.pop();
        }
    }

    return calibration_value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[googletest::test]
    fn get_digits_only_calibration_value_for_line_handles_empty() {
        expect_that!(get_digits_only_calibration_value_for_line(""), eq(0));
        expect_that!(get_digits_only_calibration_value_for_line("ob"), eq(0));
    }

    #[googletest::test]
    fn get_digits_only_calibration_value_for_line_handles_single_digit() {
        expect_that!(get_digits_only_calibration_value_for_line("1"), eq(11));
        expect_that!(get_digits_only_calibration_value_for_line("a1"), eq(11));
        expect_that!(get_digits_only_calibration_value_for_line("a1a"), eq(11));
        expect_that!(
            get_digits_only_calibration_value_for_line("aaaa1aaaaa"),
            eq(11)
        );
    }

    #[googletest::test]
    fn get_digits_only_calibration_value_for_line_handles_double_digit() {
        expect_that!(get_digits_only_calibration_value_for_line("12"), eq(12));
        expect_that!(get_digits_only_calibration_value_for_line("1a2"), eq(12));
        expect_that!(get_digits_only_calibration_value_for_line("a1a2a"), eq(12));
        expect_that!(
            get_digits_only_calibration_value_for_line("a12b3ba2a"),
            eq(12)
        );
    }

    #[googletest::test]
    fn get_calibration_value_for_line_handles_empty() {
        expect_that!(get_calibration_value_for_line(""), eq(0));
        expect_that!(get_calibration_value_for_line("ob"), eq(0));
    }

    #[googletest::test]
    fn get_calibration_value_for_line_handles_single_digit() {
        expect_that!(get_calibration_value_for_line("1"), eq(11));
        expect_that!(get_calibration_value_for_line("a1"), eq(11));
        expect_that!(get_calibration_value_for_line("a1a"), eq(11));
        expect_that!(get_calibration_value_for_line("aaaa1aaaaa"), eq(11));

        expect_that!(get_calibration_value_for_line("one"), eq(11));
        expect_that!(get_calibration_value_for_line("aone"), eq(11));
        expect_that!(get_calibration_value_for_line("onea"), eq(11));
        expect_that!(get_calibration_value_for_line("onine"), eq(99));
    }

    #[googletest::test]
    fn get_calibration_value_for_line_handles_double_digit() {
        expect_that!(get_calibration_value_for_line("12"), eq(12));
        expect_that!(get_calibration_value_for_line("1a2"), eq(12));
        expect_that!(get_calibration_value_for_line("a1a2a"), eq(12));
        expect_that!(get_calibration_value_for_line("a12b3ba2a"), eq(12));

        expect_that!(get_calibration_value_for_line("oneone"), eq(11));
        expect_that!(get_calibration_value_for_line("oneaaaaone"), eq(11));
        expect_that!(get_calibration_value_for_line("abconeaaaaoneabce"), eq(11));
        expect_that!(get_calibration_value_for_line("oneintwo"), eq(12));
        expect_that!(get_calibration_value_for_line("seightwo"), eq(82));
    }
}
