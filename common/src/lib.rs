use ssh2::{Channel, Session};
use std::fmt::Display;
use std::io::{prelude::*, BufReader, BufWriter};
use std::net::TcpStream;

pub struct ChannelBuffer {
    writer: BufWriter<Box<dyn Write>>,
    reader: BufReader<Box<dyn Read>>,
}

impl ChannelBuffer {
    fn new_channel(account: &str) -> anyhow::Result<Channel> {
        let tcp = TcpStream::connect("pwnable.kr:2222")?;
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        sess.userauth_password(account, "guest")?;
        let channel = sess.channel_session()?;

        Ok(channel)
    }

    pub fn tunnelled(command: &str) -> anyhow::Result<Self> {
        let current_dir = std::env::var("CARGO_PKG_NAME")?;
        let mut channel = Self::new_channel(current_dir.as_str())?;
        channel.exec(command)?;

        Ok(Self {
            writer: BufWriter::new(Box::new(channel.clone())),
            reader: BufReader::new(Box::new(channel)),
        })
    }

    pub fn write_result(&mut self, result: &(impl Display + ?Sized)) -> anyhow::Result<()> {
        let formatted = format!("{result}\n");
        self.writer.write(formatted.as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }

    pub fn read_line(&mut self, line: &mut String) -> anyhow::Result<usize> {
        Ok(self.reader.read_line(line)?)
    }
}
