use common::*;

pub struct Fd;

fn start_fd(channel_buffer: &mut ChannelBuffer) -> anyhow::Result<()> {
    channel_buffer.write_result("LETMEWIN")?;

    let mut line = String::new();
    channel_buffer.read_line(&mut line)?;
    print!("{line}");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let account = "fd";
    let command = "./fd 4660";
    let mut channel_buffer = ChannelBuffer::tunnelled(account, command)?;
    start_fd(&mut channel_buffer)?;

    Ok(())
}
