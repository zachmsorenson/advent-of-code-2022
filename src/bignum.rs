/* Implementation based on Stackoverflow discussion here:
 * https://stackoverflow.com/questions/5318068/how-to-handle-very-large-numbers-in-java-without-using-java-math-biginteger
 */

#![allow(dead_code)]
#![allow(unused)]

use std::fmt;
use std::ops::{Add, Mul};
use itertools::Itertools;
use itertools::EitherOrBoth::{Both, Left, Right};


const BASE: i32 = 1000000000;
const BASE_DEC_DIGITS: i32 = 9;


#[derive(Debug)]
pub struct BigNum {
    digits: Vec<i32>,
}


impl BigNum {
    pub fn new(digits: Vec<i32>) -> BigNum {
        let mut result = Vec::<i32>::new();
        for digit in digits {
            result.push(digit);
        }
        BigNum{digits: result}
    }
}

fn add_digit(result: &mut Vec<i32>, idx: usize, digit: i32) {
    let sum = result[idx] + digit;
    result[idx] = sum % BASE;
    let carry = sum / BASE;
    if carry > 0 {
        if result.len() < idx + 2 {
            result.push(0);
        }
        add_digit(result, idx + 1, carry);  
    }
}

fn add_digits(result: &mut Vec<i32>, digits: &Vec<i32>) {
    for (i, digit) in digits.iter().rev().enumerate() {
        if result.len() < i + 1 {
            result.push(0);
        }
        add_digit(result, i, *digit);
    }
}

impl Add for BigNum {
    type Output = Self;

    fn add(self, other: Self) -> Self {
       let mut result = Vec::<i32>::new();
       let mut carry: i32 = 0;
       
       add_digits(&mut result, &self.digits);
       add_digits(&mut result, &other.digits);
       let result = result.into_iter().rev().collect();
       BigNum::new(result)
    }
}

impl Mul for BigNum {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
       let mut result = Vec::<i32>::new();
       let mut result_len = 0;
       let mut carry: i32 = 0;
       for (i, l) in self.digits.iter().rev().enumerate() {
           for (j, r) in other.digits.iter().rev().enumerate() {
               let product: i64 = *l as i64 * *r as i64;
               let digit: i32 = (product % (BASE as i64)) as i32;
               let carry: i32 = (product / (BASE as i64)) as i32;
          
               if result.len() < i + j + 1 {
                   result.push(0);
               }
               add_digit(&mut result, i + j, digit);
               if carry > 0 {
                   if result.len() < i + j + 2 {
                       result.push(0);
                   }
                   add_digit(&mut result, i + j + 1, carry);
               }
           }
       }
       if carry > 0 {
           result.push(carry);
       }
       let result = result.into_iter().rev().collect();
       BigNum::new(result)
    }
}

impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.digits.len() == 0 {
            write!(f, "{}", "0")
        } else {
            let mut dec_string = self.digits[0].to_string();
            for digit in &self.digits[1..] {
                dec_string = format!("{}{:09}", dec_string, digit);
            }
            write!(f, "{}", dec_string)
        }
    }
}

#[test]
fn add() {
    let v1 = vec![600000000];
    let v2 = vec![345, 500000000];
    let n1 = BigNum::new(v1);
    let n2 = BigNum::new(v2);

    let expected = BigNum::new(vec![346, 100000000]);

    assert_eq!((n1 + n2).digits, expected.digits);
}

#[test]
fn mul() {
    let v1 = vec![300000000];
    let v2 = vec![1, 200000000];
    let n1 = BigNum::new(v1);
    let n2 = BigNum::new(v2);

    let expected = BigNum::new(vec![360000000, 0]);

    assert_eq!((n1 * n2).digits, expected.digits);
}

#[test]
fn display() {
    let b = BigNum::new(vec![123, 456, 0]);

    let expected = "123000000456000000000";

    assert_eq!(expected, b.to_string());
}

