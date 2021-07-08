use crate::bigint::BigInt;

impl Clone for BigInt {
    fn clone(&self) -> BigInt {
        let result:BigInt = BigInt {
            negative: self.negative,
            data: self.data.clone()
        };
        return result;
    }
}
