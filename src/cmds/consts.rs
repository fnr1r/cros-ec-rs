use paste::paste;

macro_rules! cmd {
    ($name:ident, $ver:expr) => {
        paste! {
            pub const [<EC_CMD_ $name:upper>]: $crate::types::EcCommandInfo = $crate::types::EcCommandInfo::new_known(
                $crate::types::EcKnownCommand::[<$name:camel>],
                $ver,
            );
        }
    };
}

cmd!(proto_version, 0);
cmd!(hello, 0);
cmd!(fw_charge_limit, 0);
