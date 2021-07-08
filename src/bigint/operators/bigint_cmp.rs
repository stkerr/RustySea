use crate::bigint::BigInt;
use std::cmp::Ordering;

impl std::cmp::Ord for BigInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {


	// quickly check signs
        if self.negative && !other.negative {
            return Ordering::Less;
        }
        if !self.negative && other.negative {
            return Ordering::Greater;
        }

        // quickly check for which has more data blocks
        if self.data.len() > other.data.len() {
            return Ordering::Greater;
        }
        if self.data.len() < other.data.len() {
            return Ordering::Less;
        }

        // do actual number comparison now
        for i in 0..self.data.len() {
            if self.data[i] > other.data[i] {
                return Ordering::Greater;
            }
            if self.data[i] < other.data[i] {
                return Ordering::Less;
            }
        }

        return Ordering::Equal;
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        // quickly check signs
        if self.negative && !other.negative {
            return Some(Ordering::Less);
        }
        if !self.negative && other.negative {
            return Some(Ordering::Greater);
        }

        // quickly check for which has more data blocks
        if self.data.len() > other.data.len() {
            return Some(Ordering::Greater);
        }
        if self.data.len() < other.data.len() {
            return Some(Ordering::Less);
        }
        

        // do actual number comparison now
        for i in 0..self.data.len() {
            if self.data[i] > other.data[i] {
                return Some(Ordering::Greater);
            }
            if self.data[i] < other.data[i] {
                return Some(Ordering::Less);
            }
        }
        
        return Some(Ordering::Equal);
    }
}

