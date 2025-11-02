use regex::Regex;
use ssh2::Session;
use std::net::TcpStream;
use std::io::{ BufReader, BufWriter, prelude::* };
use std::time::Duration;

struct ChannelContext {
    writer: BufWriter<Box<dyn Write>>,
    reader: BufReader<Box<dyn Read>>,
}

impl ChannelContext {
    #[cfg(test)]
    pub fn direct_channel() -> anyhow::Result<ChannelContext> {
        let tcp = TcpStream::connect("pwnable.kr:9007")?;
        tcp.set_read_timeout(Some(Duration::from_secs(60)))?;
        let writer_box = Box::new(tcp.try_clone()?);
        let reader_box = Box::new(tcp);

        Ok(ChannelContext {
            writer: BufWriter::new(writer_box),
            reader: BufReader::new(reader_box)
        })
    }

    pub fn tunneled_channel() -> anyhow::Result<ChannelContext> {
        let mut sess = Session::new()?;
        let tcp = TcpStream::connect("pwnable.kr:2222")?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        sess.userauth_password("coin1", "guest")?;
        let channel = sess.channel_direct_tcpip("127.0.0.1", 9007, None)?;
        let writer_box = Box::new(channel.clone());
        let reader_box = Box::new(channel);

        Ok(ChannelContext {
            writer: BufWriter::new(writer_box),
            reader: BufReader::new(reader_box)
        })
    }

    fn find_nc(&mut self) -> anyhow::Result<(i32, i32)> {
        let mut line: String = String::new();
        let regex = Regex::new(r"^N=(\d+) C=(\d+)")?;
        while let Ok(_) = self.reader.read_line(&mut line) {
            print!("{line}");
            if regex.is_match(&line) {
                break;
            }
            line.clear();
        }

        let caps = regex.captures(&line).expect("unmatched");

        Ok((caps[1].parse::<i32>()?, caps[2].parse::<i32>()?))
    }

    fn find_total(&mut self, start: i32, mid: i32) -> anyhow::Result<i32> {
        let regex = Regex::new(r"^(\d+)")?;
        let ivec: Vec<String> = (start..(mid + 1)).map(|n| n.to_string()).collect();
        let ivec_to_string = ivec.join(" ") + "\n";

        self.writer.write(ivec_to_string.as_bytes())?;
        self.writer.flush()?;

        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        let total_value = &regex.captures(&line).expect("unmatched")[1];

        Ok(total_value.parse::<i32>()?)
    }

    fn print_result(&mut self, start: i32) -> anyhow::Result<()> {
        self.writer.write(format!{"{start}\n"}.as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }

    fn print_to_end(&mut self) -> anyhow::Result<()> {
        let mut line = String::new();
        while let Ok(num) = self.reader.read_line(&mut line) {
            print!("{line}");
            if num == 0 {
                break;
            }
            line.clear();
        }

        Ok(())
    }
}

fn start_coin1(channel_context: &mut ChannelContext) -> anyhow::Result<()> {
    for _ in 0..100 {
        let (n, c) = channel_context.find_nc()?;
        let (mut start, mut end) = (0, n - 1);

        for _ in 0..c {
            let mid = (end + start) / 2;
            let total_value = channel_context.find_total(start, mid)?;

            if total_value % 10 == 9 {
                end = mid;
            } else {
                start = mid + 1;
            }
        }

        channel_context.print_result(start)?;
    }

    channel_context.print_to_end()?;
    
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut channel_context = ChannelContext::tunneled_channel()?;
    start_coin1(&mut channel_context)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::time::{ Instant };
    
    #[test]
    fn tunneling_speed() -> anyhow::Result<()> {
        let start = Instant::now();
        let mut channel_context = ChannelContext::tunneled_channel()?;
        start_coin1(&mut channel_context)?;

        println!("Elapsed Seconds: {}", start.elapsed().as_secs());
        Ok(())
    }

    #[test]
    fn direct_speed() -> anyhow::Result<()> {
        let start = Instant::now();
        let mut channel_context = ChannelContext::direct_channel()?;
        start_coin1(&mut channel_context)?;

        println!("Elapsed Seconds: {}", start.elapsed().as_secs());
        Ok(())
    }
}