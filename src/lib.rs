pub mod types;
pub mod logging;
pub mod interop;

use std::error::Error;

use crate::{interop::imports::{get_domain, self}, types::{common::ArbitraryData, object::UnityObject}};

fn run() -> Result<(), Box<dyn Error>> {
    log!("Hello from wasm!");

    let domain = get_domain()?;

    let assembly = domain.get_assembly("UnityEngine.CoreModule")?;

    log!(assembly.get_name()?);

    let class = assembly.get_class("UnityEngine", "Application")?;

    log!(class.get_name()?);

    let prop = class.get_property("targetFrameRate")?;

    log!(prop.get_name()?);

    let value = 1_i32;
    let value = value.to_le_bytes();

    let mut data: Vec<Vec<u8>> = Vec::new();
    data.push(value.to_vec());

    prop.set_value(None, &ArbitraryData{
        data
    })?;

    Ok(())
}