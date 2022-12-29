use crate::{BigInt, FromPrimitive};
pub fn big_pow(base: BigInt, exp: BigInt) -> BigInt
{
    if exp == BigInt::from_u32(0).unwrap()
    {
        return BigInt::from_u32(1).unwrap();
    }
    let mut answer = base.clone();
    let mut i = BigInt::from_u32(1).unwrap();
    while i < exp
    {
        i = i + BigInt::from_u32(1).unwrap();
        answer = answer * base.clone();
    }
    return answer;
}
