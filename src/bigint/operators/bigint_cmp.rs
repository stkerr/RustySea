use ::bigint::BigInt;
use std::cmp::Ordering;

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        // quickly check signs
        if self.negative && !other.negative {
            return Some(Ordering::Less);
        }
        if !self.negative && other.negative {
            return Some(Ordering::Greater);
        }

        // do actual number comparison now
        return None;
    }
}

