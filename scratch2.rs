use rodio::{Sink, OutputStream, Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
fn main() {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    
    // Create a dummy file if you want or just use try_seek and get_pos
    let _ = sink.try_seek(Duration::from_secs(10));
    println!("pos: {:?}", sink.get_pos());
}
