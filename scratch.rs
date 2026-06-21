use rodio::{Sink, OutputStream};
fn main() {
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    let _ = sink.try_seek(std::time::Duration::from_secs(10));
}
