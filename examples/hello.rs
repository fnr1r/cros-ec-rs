use anyhow::Result;
use cros_ec::{
    cmds::{hello::ec_cmd_hello, proto_version::ec_cmd_proto_version},
    interfaces::dev::EcDev,
};

fn main() -> Result<()> {
    let ec_dev = EcDev::open_cros_ec()?;
    eprintln!("EC opened!");

    ec_cmd_hello(&ec_dev)?;
    eprintln!("EC Hello successful!");

    let proto_version = ec_cmd_proto_version(&ec_dev)?;
    eprintln!("EC Protocol Version: 0x{:08x}", proto_version);

    Ok(())
}
