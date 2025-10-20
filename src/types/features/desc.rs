use super::EcFeature;

macro_rules! map_fn {
    (
        $fn_vis:vis const fn $fn_name:ident($param:ident: $param_type:ty) -> $ret_type:ty;
        $($variant:pat => $desc:expr),* $(,)?
    ) => {
        $fn_vis const fn $fn_name($param: $param_type) -> $ret_type {
            use $param_type as E;
            match $param {
                $($variant => $desc),*
            }
        }
    };
}

map_fn! {
    pub const fn ec_feature_desc(feature: EcFeature) -> &'static str;
    E::Limited => "Limited image, load RW for more",
    E::Flash => "Flash",
    E::PwmFan => "Direct Fan power management",
    E::PwmKeyb => "Keyboard backlight",
    E::Lightbar => "Lightbar",
    E::Led => "LED",
    E::MotionSense => "Motion Sensors",
    E::Keyb => "Keyboard",
    E::Pstore => "Host Permanent Storage",
    E::Port80 => "BIOS Port 80h access",
    E::Thermal => "Thermal management",
    E::BklightSwitch => "Switch backlight on/off",
    E::WifiSwitch => "Switch wifi on/off",
    E::HostEvents => "Host event",
    E::Gpio => "GPIO",
    E::I2c => "I2C controller",
    E::Charger => "Charger",
    E::Battery => "Simple Battery",
    E::SmartBattery => "Smart Battery",
    E::HangDetect => "Host hang detection",
    E::Pmu => "Power Management",
    E::SubMcu => "Control downstream MCU",
    E::UsbPd => "USB Cros Power Delivery",
    E::UsbMux => "USB Multiplexer",
    E::MotionSenseFifo => "FIFO for Motion Sensors events",
    E::VStore => "Temporary secure vstore",
    E::UsbCSsMuxVirtual => "Host-controlled USB-C SS mux",
    E::Rtc => "Real-time clock",
    E::Fingerprint => "Fingerprint",
    E::Touchpad => "Touchpad",
    E::Rwsig => "RWSIG task",
    E::DeviceEvent => "Device events reporting",
    E::UnifiedWakeMasks => "Unified wake masks for LPC/eSPI",
    E::HostEvent64 => "64-bit host events",
    E::ExecInRam => "Execute code in RAM",
    E::Cec => "Consumer Electronics Control",
    E::MotionSenseTightTimestamps => "Tight timestamp for sensors events",
    E::RefinedTabletModeHysteresis => "Refined tablet mode hysteresis",
    E::Efs2 => "Early Firmware Selection v2",
    E::Scp => "System Companion Processor",
    E::Ish => "Intel Integrated Sensor Hub",
    E::TypeCCmd => "TCPMv2 Type-C commands",
    E::TypeCRequireApModeEntry => "Host-controlled Type-C mode entry",
    E::TypeCMuxRequireApAck => "AP ack for Type-C mux configuration",
    E::S4Residency => "S4 residency",
    E::TypeCApMuxSet => "AP directed mux sets",
}
