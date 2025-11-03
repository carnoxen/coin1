use regex::Regex;
use ssh2::Session;
use std::net::TcpStream;
use std::io::{ BufReader, BufWriter, prelude::* };

struct ChannelContext {
    writer: BufWriter<Box<dyn Write>>,
    reader: BufReader<Box<dyn Read>>,
}

impl ChannelContext {
    #[cfg(test)]
    pub fn direct_channel(sec: u64) -> anyhow::Result<Self> {
        use std::time::Duration;
        let tcp = TcpStream::connect("pwnable.kr:9007")?;
        tcp.set_read_timeout(Some(Duration::from_secs(sec)))?;

        Ok(Self {
            writer: BufWriter::new(Box::new(tcp.try_clone()?)),
            reader: BufReader::new(Box::new(tcp))
        })
    }

    pub fn tunneled_channel() -> anyhow::Result<Self> {
        let mut sess = Session::new()?;
        let tcp = TcpStream::connect("pwnable.kr:2222")?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        sess.userauth_password("coin1", "guest")?;
        let channel = sess.channel_direct_tcpip("0", 9007, None)?;

        Ok(Self {
            writer: BufWriter::new(Box::new(channel.clone())),
            reader: BufReader::new(Box::new(channel))
        })
    }

    fn find_nc(&mut self) -> anyhow::Result<(i32, i32)> {
        let mut line: String = String::new();
        let regex = Regex::new(r"^N=(\d+) C=(\d+)")?;

        while let Ok(num) = self.reader.read_line(&mut line) && 
            !regex.is_match(&line) &&
            num != 0
        {
            print!("{line}");
            line.clear();
        }

        let captures = regex.captures(&line);
        match captures {
            Some(result) => Ok((result[1].parse::<i32>()?, result[2].parse::<i32>()?)),
            _ => panic!("Unmatched finding nc: {line}")
        }
    }

    fn find_total(&mut self, start: i32, mid: i32) -> anyhow::Result<i32> {
        let ivec = (start..(mid + 1)).map(|n| n.to_string()).collect::<Vec<String>>();
        let ivec_joined = ivec.join(" ") + "\n";

        self.writer.write(ivec_joined.as_bytes())?;
        self.writer.flush()?;

        let mut line = String::new();
        let regex = Regex::new(r"^(\d+)")?;

        self.reader.read_line(&mut line)?;
        let captures = &regex.captures(&line);
        match captures {
            Some(result) => Ok(result[1].parse::<i32>()?),
            _ => panic!("Unmatched finding total: {line}")
        }
    }

    fn write_result(&mut self, start: i32) -> anyhow::Result<()> {
        self.writer.write(format!("{start}\n").as_bytes())?;
        self.writer.flush()?;

        Ok(())
    }

    fn print_to_end(&mut self) -> anyhow::Result<()> {
        let mut line = String::new();

        while let Ok(num) = self.reader.read_line(&mut line) &&
            num != 0 
        {
            print!("{line}");
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

        channel_context.write_result(start)?;
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
    use std::time::Instant;
    
    #[test]
    fn tunneling_speed() -> anyhow::Result<()> {
        let start = Instant::now();
        let mut channel_context = ChannelContext::tunneled_channel()?;
        start_coin1(&mut channel_context)?;

        println!("Elapsed Seconds: {}", start.elapsed().as_secs());
        Ok(())
    }

    #[test]
    #[should_panic]
    fn direct_speed() {
        ChannelContext::direct_channel(60).unwrap();
    }
}