extern crate bzip2;

use std::io::{Cursor, Read};
use std::time::Instant;

use bzip2::read::BzDecoder;


fn main() -> Result<(), std::io::Error> {
    let compressed_bytes = include_bytes!("../sample.bz2");
    // Create a cursor to read binary data from the byte array
    let cursor = Cursor::new(compressed_bytes);

    let mut decompressor = BzDecoder::new(cursor);
    let mut total_out: usize = 0;

    let start = Instant::now();

    loop {
        let mut buf = [0; 1024]; // Read up to 1024 bytes at a time
        let bytes_read = decompressor.read(&mut buf)?;
        total_out += bytes_read;
        if bytes_read == 0 {
            break;
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed for decompression is: {:?}", duration);

    println!("decompressed {} MB", total_out as f32 / 1024.0 / 1024.0);

    return Ok(());

}