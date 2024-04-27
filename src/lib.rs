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
// this function will recursivly call it self until it it reaches one.
pub fn multi_step_recursive_colatz(num: usize, print: bool) -> usize {
    if num == 1 {
        return 1;
    }
    if num % 2 != 0 {
        if print == true {
            println!("{}", num);
        }
        return multi_step_recursive_colatz(3 * num + 1, print);
    } else {
        if print == true {
            println!("{}", num);
        }
        return multi_step_recursive_colatz(num / 2, print);
    }
}
pub fn multi_step_nonrecursive_colatz(num: usize, print: bool) -> None {
    while num != 1 {
        if num % 2 != 0 {
            if print == true {
                println!("{}", num);
            }
        } else {
            if print == true {
                println!("{}", num);
            }
        }
    }
}
