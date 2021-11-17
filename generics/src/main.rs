use std::io::{BufWriter, Write};
use std::net::TcpStream;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
struct MyWrter<W> {
    writer: W,
}

impl<W> MyWrter<W> where W: Write {
    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

impl MyWrter<BufWriter<TcpStream>> {
    pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect(addr).unwrap();
        Self {
            writer: BufWriter::new(stream),
        }
    }

}

impl MyWrter<File> {
    pub fn new(addr: &str) -> Self {
        let path = Path::new(addr);
        let file = File::create(path).unwrap();
        Self {
            writer: file,
        }
    }
}

fn main() {
    // let mut writer = MyWrter::<BufWriter<TcpStream>>::new("127.0.0.1:8080");
    // writer.write("hello world!").unwrap();

    let mut writer2 = MyWrter::<File>::new("hello.txt");
    writer2.write("hello world").unwrap();
}
