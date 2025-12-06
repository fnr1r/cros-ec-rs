use enumflags2::bitflags;

#[bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EcFlashInfoFlags {
    /// EC flash erases bits to 0 instead of 1.
    ///
    /// Flags for version 1+ flash info command
    EraseToZero = 1 << 0,
    /// Flash must be selected for read/write/erase operations to succeed.
    ///
    /// This may be necessary on a chip where write/erase can be corrupted by
    /// other board activity, or where the chip needs to enable some sort of
    /// programming voltage, or where the read/write/erase operations require
    /// cleanly suspending other chip functionality.
    SelectRequired = 1 << 1,
}
