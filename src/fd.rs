use crate::common::*;

pub struct Fd;

impl Run for Fd {
    fn run() -> anyhow::Result<()> {
        println!("== FD START!");

        let account = "fd";
        let command = "./fd 4660";
        let mut channel_buffer = ChannelBuffer::tunnelled(account, command)?;
        channel_buffer.write_result("LETMEWIN".to_string())?;

        let mut line = String::new();
        channel_buffer.read_line(&mut line)?;
        print!("Result:{line}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {}