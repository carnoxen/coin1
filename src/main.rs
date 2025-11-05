use crate::{coin1::Coin1, common::Run};

mod coin1;
mod common;

fn main() -> anyhow::Result<()> {
    Coin1::run()?;

    Ok(())
}