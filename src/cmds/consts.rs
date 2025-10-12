use paste::paste;

macro_rules! cmd {
    ($name:ident, $cmd:expr, $ver:expr) => {
        paste! {
            pub const [<EC_CMD_ $name:upper>]: $crate::types::EcCommandMeta = $crate::types::EcCommandMeta::new($cmd, $ver, stringify!($name));
        }
    };
}
/*macro_rules! cmds {
    ($($name:tt, $cmd:expr, $ver:expr;)*) => {
        $(cmd!($name, $cmd, $ver);)*
    };
}*/

cmd!(hello, 1, 0);
cmd!(fw_charge_limit, 0x3E03, 0);
