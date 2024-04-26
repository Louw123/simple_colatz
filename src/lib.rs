//! # Simple colatz
//! simple is a group of crate that simplifies functions for beginners
//! with clean non-terse documentation.
//! This package allows you to compute the colatz conjecture

#[doc = r" this function uses a number computes the next number by useing a if condition: If the number doesent have a remainder when its divided by 2 then, divide the number by 2, and return said number, else we multipy and add 1 to the number and return said number."]
pub fn single_step_colatz(num: usize) -> usize {
    if num % 2 != 0 {
        return 3 * num + 1;
    } else {
        return num / 2;
    }
}
pub fn multi_step_recursive_colatz(num: usize) -> usize {
    if num == 1 {
        return 1;
    }
    if num % 2 != 0 {
        return multi_step_recursive_colatz(3 * num + 1);
    } else {
        return multi_step_recursive_colatz(num / 2);
    }
}
#[cfg(test)]
mod test {
    use super::single_step_colatz;
    #[test]
    fn colup() {
        assert_eq!(single_step_colatz(3), 10);
    }
}
