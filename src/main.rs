use regex::Regex;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::Instant;

struct NC(u32, u32);

fn escapetonc(reader: &mut BufReader<&TcpStream>, regex: &Regex, line: &mut String) {
    let now = Instant::now();

    loop {
        line.clear();
        reader.read_line(line).unwrap();
        print!("{}", &line);
        if regex.is_match(&line) {
            break;
        }
        if now.elapsed().as_secs() >= 30u64 {
            panic!("coin1 game ended");
        }
    }
}

fn findnc(s: &String, r: &Regex) -> NC {
    let caps = r.captures(&s).unwrap();

    let n = caps.name("n").unwrap().as_str().parse::<u32>().unwrap();
    let c = caps.name("c").unwrap().as_str().parse::<u32>().unwrap();
    NC(n, c)
}

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("pwnable.kr:9007")?;
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let regex = Regex::new(r"^N=(?P<n>\d+) C=(?P<c>\d+)").unwrap();
    let regex2 = Regex::new(r"^(\d+)").unwrap();

    let mut times = 110;
    let mut line = String::new();
    while times > 0 {
        escapetonc(&mut reader, &regex, &mut line);
        let nc = findnc(&line, &regex);
        let n = nc.0;
        let c = nc.1;

        let (mut start, mut end) = (0u32, n - 1u32);
        for _ in 0u32..c {
            let mid = (end + start) / 2u32;

            let stringvector: Vec<String> = (start..(mid + 1u32)).map(|u| u.to_string()).collect();
            let mut sending_string = stringvector.join(" ");
            sending_string.push('\n');

            writer.write(sending_string.as_bytes())?;
            writer.flush()?;

            line.clear();
            reader.read_line(&mut line)?;

            let total_value = regex2.captures(&line).unwrap().get(1).unwrap().as_str().trim().parse::<u32>().unwrap();
            if total_value % 10u32 == 9u32 {
                end = mid;
            } else {
                start = mid + 1u32;
            }
        }

        writer.write(format!{"{}\n", start.to_string()}.as_bytes())?;
        writer.flush()?;

        times = times - 1;
    }
    Ok(())
} // the stream is closed here