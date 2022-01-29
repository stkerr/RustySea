use crate::bigint::BigInt;

use std::fmt::*;

impl Display for BigInt {

    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut data_copy:Vec<u64> = self.data.clone();
        data_copy.reverse();
 
        if self.negative {
            let result:Result = f.write_fmt(format_args!("-"));
            match result {
                Ok(v) => v,
                Err(e) => std::panic::panic_any(e)
            }
        }

        let mut result:Result = f.write_fmt(format_args!("0x"));
        match result {
            Ok(v) => v,
            Err(e) => std::panic::panic_any(e)
        }

        if data_copy.len() == 0 {
            result = f.write_str("0");
            match result {
                Ok(v) => v,
                Err(e) => std::panic::panic_any(e)
            }
        }
        else {
            for i in data_copy {
                result = f.write_fmt(format_args!("{:016x}",i));
                match result {
                    Ok(v) => v,
                    Err(e) => std::panic::panic_any(e)
                }
            }
        }
        return Ok(());
    }
}
