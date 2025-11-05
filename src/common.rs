use ssh2::Session;
use std::net::TcpStream;

pub use ssh2::Channel;

pub fn new_channel(account: &str) -> anyhow::Result<Channel> {
    let tcp = TcpStream::connect("pwnable.kr:2222")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password(account, "guest")?;
    let channel = sess.channel_session()?;

    Ok(channel)
}

pub trait Run {
    fn run() -> anyhow::Result<()>;
}