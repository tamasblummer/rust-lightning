// This file is auto-generated by gen_target.sh based on msg_target_template.txt
// To modify it, modify msg_target_template.txt and run gen_target.sh instead.

extern crate bitcoin;
extern crate bitcoin_hashes;
extern crate lightning;

use bitcoin_hashes::sha256d::Hash as Sha256dHash;

use lightning::ln::channelmonitor;
use lightning::util::reset_rng_state;
use lightning::util::ser::{ReadableArgs, Writer};

mod utils;
use utils::test_logger;

use std::io::Cursor;
use std::sync::Arc;

struct VecWriter(Vec<u8>);
impl Writer for VecWriter {
	fn write_all(&mut self, buf: &[u8]) -> Result<(), ::std::io::Error> {
		self.0.extend_from_slice(buf);
		Ok(())
	}
	fn size_hint(&mut self, size: usize) {
		self.0.reserve_exact(size);
	}
}

#[inline]
pub fn do_test(data: &[u8]) {
	reset_rng_state();
	let logger = Arc::new(test_logger::TestLogger::new("".to_owned()));
	if let Ok((latest_block_hash, monitor)) = <(Sha256dHash, channelmonitor::ChannelMonitor)>::read(&mut Cursor::new(data), logger.clone()) {
		let mut w = VecWriter(Vec::new());
		monitor.write_for_disk(&mut w).unwrap();
		let deserialized_copy = <(Sha256dHash, channelmonitor::ChannelMonitor)>::read(&mut Cursor::new(&w.0), logger.clone()).unwrap();
		assert!(latest_block_hash == deserialized_copy.0);
		assert!(monitor == deserialized_copy.1);
		w.0.clear();
		monitor.write_for_watchtower(&mut w).unwrap();
	}
}

#[cfg(feature = "afl")]
#[macro_use] extern crate afl;
#[cfg(feature = "afl")]
fn main() {
	fuzz!(|data| {
		do_test(data);
	});
}

#[cfg(feature = "honggfuzz")]
#[macro_use] extern crate honggfuzz;
#[cfg(feature = "honggfuzz")]
fn main() {
	loop {
		fuzz!(|data| {
			do_test(data);
		});
	}
}

extern crate hex;
#[cfg(test)]
mod tests {

	#[test]
	fn duplicate_crash() {
		super::do_test(&::hex::decode("00").unwrap());
	}
}
