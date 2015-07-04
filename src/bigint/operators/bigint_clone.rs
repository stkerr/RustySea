use ::bigint::BigInt;

impl Clone for BigInt {
    fn clone(&self) -> BigInt {
        let result:BigInt = BigInt {
            length: self.length,
            negative: self.negative,
            data: self.data.clone()
        };
        return result;
    }
}
