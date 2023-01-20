#![allow(unused_assignments)]
#![allow(non_snake_case)]
use num_bigint::BigInt;
use std::env;
use std::str::FromStr;
fn main() 
{
    //divide int by 16
    //get int quotient(ans) for next iter
    //get the remainder for the hex digit
    //repeat until ans = 0
    let bighexstring = int_to_hex(BigInt::from_str(&env::args().last().unwrap().to_string()).expect("\n\n! please provide big int as an argument !\n\n"));
    dbg!(&bighexstring);
}
fn int_to_hex(int: BigInt) -> String
{
    let DEBUG = false;
    let mut hex_string: String = String::new();
    let mut ans = int;
    let mut _16: BigInt = BigInt::from(16);
    while &ans !=&BigInt::from(0)
    {
        let mut hex_digit = String::new();
        if DEBUG == true {println!("Quotient:  {}", ans);}
        let quo = ans.clone();
        ans = &ans / &_16;
        let rem  = quo % &_16;
        if DEBUG == true {println!("Remainder: {}", rem);}
        if rem == BigInt::from(10)
        {
            hex_digit = "a".to_string();
        }
        else if rem == BigInt::from(11)
        {
            hex_digit = "b".to_string();
        }
        else if rem == BigInt::from(12)
        {
            hex_digit = "c".to_string();
        }
        else if rem == BigInt::from(13)
        {
            hex_digit = "d".to_string();
        }
        else if rem == BigInt::from(14)
        {
            hex_digit = "e".to_string();
        }
        else if rem == BigInt::from(15)
        {
            hex_digit = "f".to_string();
        }
        else 
        {
            hex_digit = rem.to_string();
        }
        hex_string =  hex_digit + &hex_string;
        if DEBUG == true {println!("Current Hex String:   {}", hex_string);}
    }
    return hex_string;
}
