use std::sync::LazyLock;

use crate::common::*;
use regex::Regex;

fn find_nc(channel_buffer: &mut ChannelBuffer) -> anyhow::Result<(i32, i32)> {
    let mut line: String = String::new();
    static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^N=(\d+) C=(\d+)").unwrap());

    while let Ok(num) = channel_buffer.read_line(&mut line)
        && !REGEX.is_match(&line)
        && num != 0
    {
        print!("{line}");
        line.clear();
    }

    let captures = REGEX.captures(&line)
        .expect("Unmatched finding nc");
    Ok((captures[1].parse()?, captures[2].parse()?))
}

fn find_total(channel_buffer: &mut ChannelBuffer, start: i32, mid: i32) -> anyhow::Result<i32> {
    let ivec = (start..(mid + 1))
        .map(|n| n.to_string())
        .collect::<Vec<String>>();
    let ivec_joined = ivec.join(" ");
    channel_buffer.write_result(&ivec_joined)?;

    let mut line = String::new();
    channel_buffer.read_line(&mut line)?;
    line = line.trim().to_string();

    Ok(line.parse::<i32>()?)
}

fn print_to_end(channel_buffer: &mut ChannelBuffer) -> anyhow::Result<()> {
    let mut line = String::new();
    for _ in 0..3 {
        channel_buffer.read_line(&mut line)?;
        print!("{line}");
        line.clear();
    }

    Ok(())
}

fn start_coin1(channel_buffer: &mut ChannelBuffer) -> anyhow::Result<()> {
    for _ in 0..100 {
        let (n, c) = find_nc(channel_buffer)?;
        let (mut start, mut end) = (0, n - 1);

        for _ in 0..c {
            let mid = (end + start) / 2;
            let total_value = find_total(channel_buffer, start, mid)?;

            if total_value % 10 == 9 {
                end = mid;
            } else {
                start = mid + 1;
            }
        }

        let result_string = start.to_string();
        channel_buffer.write_result(&result_string)?;
    }

    print_to_end(channel_buffer)?;

    Ok(())
}

pub struct Coin1;

impl Run for Coin1 {
    fn run() -> anyhow::Result<()> {
        println!("== COIN1 START!");

        let account = "coin1";
        let command = "nc 0 9007";
        let mut channel_buffer = ChannelBuffer::tunnelled(account, command)?;
        start_coin1(&mut channel_buffer)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::coin1::*;
    use std::time::Instant;

    #[test]
    fn tunnelling_speed() -> anyhow::Result<()> {
        let start = Instant::now();
        Coin1::run()?;

        println!("Elapsed Seconds: {}", start.elapsed().as_secs());
        Ok(())
    }
}
