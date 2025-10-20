use enumflags2::bitflags;

#[bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum EcFeature {
    /// This image contains a limited set of features.
    ///
    /// Another image in RW partition may support more features.
    Limited = 1 << 0,
    /// Commands for probing/reading/writing/erasing the flash in the EC
    /// are present.
    Flash = 1 << 1,
    /// Can control the fan speed directly.
    PwmFan = 1 << 2,
    /// Can control the intensity of the keyboard backlight.
    PwmKeyb = 1 << 3,
    /// Support Google lightbar, introduced on Pixel.
    Lightbar = 1 << 4,
    /// Control of LEDs
    Led = 1 << 5,
    /// Exposes an interface to control gyro and sensors.
    ///
    /// The host goes through the EC to access these sensors.
    /// In addition, the EC may provide composite sensors, like lid angle.
    MotionSense = 1 << 6,
    /// The keyboard is controlled by the EC
    Keyb = 1 << 7,
    /// The AP can use part of the EC flash as persistent storage.
    Pstore = 1 << 8,
    /// The EC monitors BIOS port 80h, and can return POST codes.
    Port80 = 1 << 9,
    /// Thermal management: include TMP specific commands.
    ///
    /// Higher level than direct fan control.
    Thermal = 1 << 10,
    /// Can switch the screen backlight on/off
    BklightSwitch = 1 << 11,
    /// Can switch the wifi module on/off
    WifiSwitch = 1 << 12,
    /// Monitor host events, through for example SMI or SCI
    HostEvents = 1 << 13,
    /// The EC exposes GPIO commands to control/monitor connected devices.
    Gpio = 1 << 14,
    /// The EC can send i2c messages to downstream devices.
    I2c = 1 << 15,
    /// Command to control charger are included
    Charger = 1 << 16,
    /// Simple battery support.
    Battery = 1 << 17,
    /// Support Smart battery protocol
    ///
    /// (Common Smart Battery System Interface Specification)
    SmartBattery = 1 << 18,
    /// EC can detect when the host hangs.
    HangDetect = 1 << 19,
    /// Report power information, for pit only
    Pmu = 1 << 20,
    /// Another Cros EC device is present downstream of this one
    SubMcu = 1 << 21,
    /// Support USB Power delivery (PD) commands
    UsbPd = 1 << 22,
    /// Control USB multiplexer, for audio through USB port for instance.
    UsbMux = 1 << 23,
    /// Motion Sensor code has an internal software FIFO
    MotionSenseFifo = 1 << 24,
    /// Support temporary secure vstore
    VStore = 1 << 25,
    /// EC decides on USB-C SS mux state, muxes configured by host
    UsbCSsMuxVirtual = 1 << 26,
    /// EC has RTC feature that can be controlled by host commands
    Rtc = 1 << 27,
    /// The MCU exposes a Fingerprint sensor
    Fingerprint = 1 << 28,
    /// The MCU exposes a Touchpad
    Touchpad = 1 << 29,
    /// The MCU has RWSIG task enabled
    Rwsig = 1 << 30,
    /// EC has device events support
    DeviceEvent = 1 << 31,
    /// EC supports the unified wake masks for LPC/eSPI systems
    UnifiedWakeMasks = 1 << 32,
    /// EC supports 64-bit host events
    HostEvent64 = 1 << 33,
    /// EC runs code in RAM (not in place, a.k.a. XIP)
    ExecInRam = 1 << 34,
    /// EC supports CEC commands
    Cec = 1 << 35,
    /// EC supports tight sensor timestamping.
    MotionSenseTightTimestamps = 1 << 36,
    /// EC supports tablet mode detection aligned to Chrome and allows
    /// setting of threshold by host command using
    /// MOTIONSENSE_CMD_TABLET_MODE_LID_ANGLE.
    RefinedTabletModeHysteresis = 1 << 37,
    /// Early Firmware Selection ver.2. Enabled by CONFIG_VBOOT_EFS2.
    ///
    /// Note this is a RO feature. So, a query (EC_CMD_GET_FEATURES) should
    /// be sent to RO to be precise.
    Efs2 = 1 << 38,
    /// The MCU is a System Companion Processor (SCP).
    Scp = 1 << 39,
    /// The MCU is an Integrated Sensor Hub
    Ish = 1 << 40,
    /// New TCPMv2 TYPEC_ prefaced commands supported
    TypeCCmd = 1 << 41,
    /// The EC will wait for direction from the AP to enter Type-C alternate
    /// modes or USB4.
    TypeCRequireApModeEntry = 1 << 42,
    /// The EC will wait for an acknowledge from the AP after setting the
    /// mux.
    TypeCMuxRequireApAck = 1 << 43,
    /// The EC supports entering and residing in S4.
    S4Residency = 1 << 44,
    /// The EC supports the AP directing mux sets for the board.
    TypeCApMuxSet = 1 << 45,
}
