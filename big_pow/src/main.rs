use num_bigint::BigInt;
use num_traits::{cast::FromPrimitive};
fn main() 
{
    println!("{}", big_pow(BigInt::from_u32(256).unwrap(), BigInt::from_u32(256).unwrap()));   
    //really huge  numbers take a while but will work given enough time to loop
}

pub fn big_pow(base: BigInt, exp: BigInt) -> BigInt
{
    let big0 = BigInt::from_u32(0).unwrap();
    let big1 = BigInt::from_u32(1).unwrap();
    if exp == big0.clone()
    {
        return big1.clone();
    }
    let mut answer = base.clone();
    let mut i = big1.clone();
    while i < exp
    {
        i = i + big1.clone();
        answer = answer * base.clone();
    }
    return answer;
}

