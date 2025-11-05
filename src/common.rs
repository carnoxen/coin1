use std::fmt::{Display};
use std::net::TcpStream;
use std::io::{ BufReader, BufWriter, prelude::* };
use ssh2::{Session, Channel};


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

    pub fn tunnelled(account: &str, command: &str) -> anyhow::Result<Self> {
        let mut channel = Self::new_channel(account)?;
        channel.exec(command)?;

        Ok(Self {
            writer: BufWriter::new(Box::new(channel.clone())),
            reader: BufReader::new(Box::new(channel))
        })
    }

    pub fn write_result(&mut self, result: &(impl Display + ?Sized)) -> anyhow::Result<()> {
        self.writer.write(format!("{result}\n").as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }

    pub fn read_line(&mut self, line: &mut String) -> anyhow::Result<usize> {
        Ok(self.reader.read_line(line)?)
    }
}

pub trait Run {
    fn run() -> anyhow::Result<()>;
}