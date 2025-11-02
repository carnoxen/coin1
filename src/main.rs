use regex::Regex;
use ssh2::Channel;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

fn findnc(line: &mut String, reader: &mut BufReader<Channel>) -> anyhow::Result<(i32, i32)> {
    let regex = Regex::new(r"^N=(\d+) C=(\d+)")?;
    while !regex.is_match(&line) {
        line.clear();
        reader.read_line(line)?;
        print!("{line}");
    }

    let (_, [n, c]) = regex.captures(&line).map(|c| c.extract()).expect("unmatch");

    Ok((n.parse::<i32>()?, c.parse::<i32>()?))
}

fn tunneled_channel() -> anyhow::Result<Channel> {
    use ssh2::Session;

    let tcp = TcpStream::connect("pwnable.kr:2222")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_password("coin1", "guest")?;

    Ok(sess.channel_direct_tcpip("127.0.0.1", 9007, None)?)
}

fn main() -> anyhow::Result<()> {
    let channel = tunneled_channel()?;
    let mut writer = BufWriter::new(channel.clone());
    let mut reader = BufReader::new(channel);
    let mut line = String::new();

    for _ in 0..100 {
        let nc = findnc(&mut line, &mut reader)?;
        let (mut start, mut end) = (0, nc.0 - 1);

        for _ in 0..nc.1 {
            let mid = (end + start) / 2;

            let stringvector: Vec<String> = (start..(mid + 1)).map(|u| u.to_string()).collect();
            let sending_string = stringvector.join(" ") + "\n";

            writer.write(sending_string.as_bytes())?;
            writer.flush()?;

            line.clear();
            reader.read_line(&mut line)?;

            let regex2 = Regex::new(r"^(\d+)")?;
            let (_, [total_value]) = regex2.captures(&line).map(|c| c.extract()).expect("unmatched");
            if total_value.parse::<i32>()? % 10 == 9 {
                end = mid;
            } else {
                start = mid + 1;
            }
        }

        writer.write(format!{"{start}\n"}.as_bytes())?;
        writer.flush()?;
    }

    line.clear();
    while let Ok(num) = reader.read_line(&mut line) {
        print!("{line}");
        line.clear();

        if num == 0 {
            break;
        }
    }

    Ok(())
}