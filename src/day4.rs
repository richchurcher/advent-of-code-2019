use aoc_runner_derive::aoc;
use itertools::multipeek;

pub fn has_doubles(password: &str) -> bool {
    let mut letters = password.chars().peekable();
    while let Some(letter) = letters.next() {
        let is_double = match letters.peek() {
            Some(x) => *x == letter,
            None => false,
        };
        if is_double {
            return true;
        }
    }

    false
}

pub fn has_doubles_without_larger_group(password: &str) -> bool {
    let mut letters = multipeek(password.chars());
    while let Some(letter) = letters.next() {
        let is_double = match letters.peek() {
            Some(x) => *x == letter,
            None => false,
        };
        if is_double {
            let is_larger_group = match letters.peek() {
                Some(x) => *x == letter,
                None => false,
            };

            if !is_larger_group {
                return true;
            }

            letters.next();

            while let Some(next) = letters.peek() {
                if *next != letter {
                    break;
                }
                letters.next();
            }
        }
    }

    false
}

pub fn always_increasing(password: &str) -> bool {
    let mut letters = password.chars().peekable();
    while let Some(letter) = letters.next() {
        let is_greater = match letters.peek() {
            Some(x) => x.to_digit(10).unwrap() >= letter.to_digit(10).unwrap(),
            None => true,
        };
        if !is_greater {
            return false;
        }
    }

    true
}

#[aoc(day4, part1)]
pub fn find_password_combinations(input: &str) -> usize {
    let range: Vec<i32> = input
        .split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut passwords = 0;
    for password in range[0]..=range[1] {
        let s = &password.to_string();
        if !has_doubles(s) {
            continue;
        }

        if always_increasing(s) {
            passwords += 1;
        }
    }

    passwords
}

#[aoc(day4, part2)]
pub fn find_password_combinations_one_more_detail(input: &str) -> usize {
    let range: Vec<i32> = input
        .split('-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut passwords = 0;
    for password in range[0]..=range[1] {
        let s = &password.to_string();
        if !has_doubles_without_larger_group(s) {
            continue;
        }

        if always_increasing(s) {
            passwords += 1;
        }
    }

    passwords
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn always_increasing_correct_for_true() {
        assert_eq!(always_increasing("12345"), true)
    }

    #[test]
    fn always_increasing_correct_for_false() {
        assert_eq!(always_increasing("12315"), false)
    }

    #[test]
    fn rule_out_larger_groups_appropriately() {
        assert_eq!(has_doubles_without_larger_group("111234"), false)
    }

    #[test]
    fn allow_doubles_at_end_of_string() {
        assert_eq!(has_doubles_without_larger_group("123455"), true)
    }

    #[test]
    fn allow_doubles_without_larger_groups() {
        assert_eq!(has_doubles_without_larger_group("112345"), true)
    }

    #[test]
    fn allow_doubles_with_a_preceeding_larger_group() {
        assert_eq!(has_doubles_without_larger_group("111122"), true)
    }
}
