use ::bigint::BigInt;

use std::fmt::*;

impl Display for BigInt {

	fn fmt(&self, f: &mut Formatter) -> Result {
		let mut data_copy:Vec<u64> = self.data.clone();
		data_copy.reverse();
		
		for i in data_copy {
			// println!("i: {}", i);
			let result:Result = f.write_fmt(format_args!("{:x}",i));
			match result {
				Ok(v) => v,
				Err(e) => panic!(e)
			}
		}
		return Ok(());
	}
}