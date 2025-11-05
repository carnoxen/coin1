use regex::Regex;
use std::io::{ BufReader, BufWriter, prelude::* };
use crate::common::*;

struct ChannelContext {
    writer: BufWriter<Box<dyn Write>>,
    reader: BufReader<Box<dyn Read>>,
}

impl ChannelContext {
    fn tunneled_channel(command: &str) -> anyhow::Result<Self> {
        let mut channel = new_channel("coin1")?;
        channel.exec(command)?;

        Ok(Self {
            writer: BufWriter::new(Box::new(channel.clone())),
            reader: BufReader::new(Box::new(channel))
        })
    }

    fn write_result(&mut self, str: String) -> anyhow::Result<()> {
        self.writer.write(format!("{str}\n").as_bytes())?;
        self.writer.flush()?;

        Ok(())
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
            _ => panic!("Unmatched finding nc")
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
            _ => panic!("Unmatched finding total")
        }
    }

    fn print_to_end(&mut self) -> anyhow::Result<()> {
        let mut line = String::new();
        for _ in 0..3 {
            self.reader.read_line(&mut line)?;
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

        channel_context.write_result(start.to_string())?;
    }

    channel_context.print_to_end()?;
    
    Ok(())
}

pub struct Coin1;

impl Run for Coin1 {
    fn run() -> anyhow::Result<()> {
        let command = "nc 0 9007";
        let mut channel_context = ChannelContext::tunneled_channel(command)?;
        start_coin1(&mut channel_context)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::coin1::*;
    use std::time::Instant;
    
    #[test]
    fn tunneling_speed() -> anyhow::Result<()> {
        let start = Instant::now();
        let command = "nc 0 9007";
        let mut channel_context = ChannelContext::tunneled_channel(command)?;
        start_coin1(&mut channel_context)?;

        println!("Elapsed Seconds: {}", start.elapsed().as_secs());
        Ok(())
    }
}