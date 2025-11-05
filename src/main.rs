use crate::{coin1::Coin1, common::Run, fd::Fd};

mod fd;
mod coin1;
mod common;

fn main() -> anyhow::Result<()> {
    Fd::run()?;
    Coin1::run()?;

    Ok(())
}