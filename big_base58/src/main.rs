mod big_pow; //see: https://github.com/dzyphr/Rust_Repo/tree/master/big_pow
use big_pow::big_pow;
use num_bigint::BigInt;
use num_traits::{cast::FromPrimitive, ToPrimitive};
use std::str::FromStr;
fn main() 
{
    let _bignum = big_pow(BigInt::from_u32(6).unwrap(), BigInt::from_u32(64).unwrap()); //Generate from big_pow
    let bignum = BigInt::from_str("003287415686187229900350700442047188987144493507152461645446").unwrap(); //hard coded string
    //this implementation will not handle padded zeroes yet
    //all we have to do is count the number of them at the beginning of a string
    //(will only happen via string input, integers do not get padded) pad a '1' per two 0's,
    //generally there would only be two proceeding zeroes in a compressed bitcoin public key
    //but that is probably not an absolute guarentee
    println!("Big Number:           {}", bignum.clone());
    let b58hash = base58_encode(bignum.clone());
    println!("Base58 Hash:          {}", b58hash.clone());
    let b58hash_decode_check = base58_decode(b58hash.clone());
    println!("Decoded Base58 Hash:  {} ", b58hash_decode_check.clone());
    assert!(b58hash_decode_check == bignum.clone(), "if b58hash_decode_check != bignum then encoding or decoding failed");
}

pub fn base58_decode(hash: String) -> BigInt
{
    let mut total = BigInt::from_u32(0).unwrap();
    for p in 0..hash.len()
    {
        let base: BigInt = BigInt::from_u32(58).unwrap();
        let i: u64;
        match hash.chars().nth((hash.len()-1) - p).unwrap()
        {
            '1' => i = 0,
            '2' => i = 1,
            '3' => i = 2,
            '4' => i = 3,
            '5' => i = 4,
            '6' => i = 5,
            '7' => i = 6,
            '8' => i = 7,
            '9' => i = 8,
            'A' => i = 9,
            'B' => i = 10,
            'C' => i = 11,
            'D' => i = 12,
            'E' => i = 13,
            'F' => i = 14,
            'G' => i = 15,
            'H' => i = 16,
            'J' => i = 17,
            'K' => i = 18,
            'L' => i = 19,
            'M' => i = 20,
            'N' => i = 21,
            'P' => i = 22,
            'Q' => i = 23,
            'R' => i = 24,
            'S' => i = 25,
            'T' => i = 26,
            'U' => i = 27,
            'V' => i = 28,
            'W' => i = 29,
            'X' => i = 30,
            'Y' => i = 31,
            'Z' => i = 32,
            'a' => i = 33,
            'b' => i = 34,
            'c' => i = 35,
            'd' => i = 36,
            'e' => i = 37,
            'f' => i = 38,
            'g' => i = 39,
            'h' => i = 40,
            'i' => i = 41,
            'j' => i = 42,
            'k' => i = 43,
            'm' => i = 44,
            'n' => i = 45,
            'o' => i = 46,
            'p' => i = 47,
            'q' => i = 48,
            'r' => i = 49,
            's' => i = 50,
            't' => i = 51,
            'u' => i = 52,
            'v' => i = 53,
            'w' => i = 54,
            'x' => i = 55,
            'y' => i = 56,
            'z' => i = 57,
            x => panic!("Unexpected invalid token {:?}", x),
        }
        let big_int = BigInt::from_u64(i).unwrap();
        let c_pow = big_pow(base, BigInt::from_u32(p as u32).unwrap());
        let eq = big_int * c_pow;
        total += eq;
    }
    return total;
}

pub fn base58_encode(mut value: BigInt) -> String
{
    let base = BigInt::from_u64(58).unwrap();
    let rem = BigInt::from_u64(0).unwrap();
    let mut result: String = String::new();
    while value.clone() != rem
    {
        let rem = value.clone() % base.clone();//BigInt can handle uncompressed PublicKeys
        let rem32 = rem.to_u32().unwrap();//overflow will likely just fail on unwrap
//        dbg!(&rem); //if there was an overflow in the conversion it will show that rem is != rem32
//        //we can potentially replace the u32  in get char and just check that rem is
//        BigInt::from_u32(base58ints) => base58char
//        dbg!(&rem32);//this probably needs to be checked via a string and assert to avoid type-conversion on big int
//        push to the result before you change the value to prevent cutting off the end
        result = format!("{}{}", get_char(rem32), result);
        value = value.clone() / base.clone();
    }
    fn get_char(int: u32) -> char
    {
        let c: char;
        match int
        {
            0 => c = '1',
            1 => c = '2',
            2 => c = '3',
            3 => c = '4',
            4 => c = '5',
            5 => c = '6',
            6 => c = '7',
            7 => c = '8',
            8 => c = '9',
            9 => c = 'A',
            10 => c = 'B',
            11 => c = 'C',
            12 => c = 'D',
            13 => c = 'E',
            14 => c = 'F',
            15 => c = 'G',
            16 => c = 'H',
            17 => c = 'J',
            18 => c = 'K',
            19 => c = 'L',
            20 => c = 'M',
            21 => c = 'N',
            22 => c = 'P',
            23 => c = 'Q',
            24 => c = 'R',
            25 => c = 'S',
            26 => c = 'T',
            27 => c = 'U',
            28 => c = 'V',
            29 => c = 'W',
            30 => c = 'X',
            31 => c = 'Y',
            32 => c = 'Z',
            33 => c = 'a',
            34 => c = 'b',
            35 => c = 'c',
            36 => c = 'd',
            37 => c = 'e',
            38 => c = 'f',
            39 => c = 'g',
            40 => c = 'h',
            41 => c = 'i',
            42 => c = 'j',
            43 => c = 'k',
            44 => c = 'm',
            45 => c = 'n',
            46 => c = 'o',
            47 => c = 'p',
            48 => c = 'q',
            49 => c = 'r',
            50 => c = 's',
            51 => c = 't',
            52 => c = 'u',
            53 => c = 'v',
            54 => c = 'w',
            55 => c = 'x',
            56 => c = 'y',
            57 => c = 'z',
            x => panic!("Unexpected invalid token {:?}", x),
        }
        return c;
    }
    return result;
}

