use core::mem::MaybeUninit;

use easy_ext::ext;
use plain::Plain;
use rustix::io::Errno;

use crate::traits::EcHasReadmem;

type Result<T, E = Errno> = core::result::Result<T, E>;

#[ext(MaybeUninitPlainExt)]
impl<T: Plain> MaybeUninit<T> {
    /// Returns an uninitialized mutable slice of memory for manual init `T` is
    /// assumed to be sized.
    fn uninit_slice_mut(&mut self) -> &mut [u8] {
        use core::slice::from_raw_parts_mut;
        let data = self.as_mut_ptr();
        let len = size_of::<T>();
        unsafe { from_raw_parts_mut(data as *mut _, len) }
    }
}

fn ec_read_any<T, U>(this: &T, offset: i32) -> Result<U>
where
    T: ?Sized + EcHasReadmem,
    U: Plain,
{
    let mut res = MaybeUninit::<U>::uninit();
    let len = {
        let res = res.uninit_slice_mut();
        T::ec_readmem(this, offset, res)?
    };
    let res = unsafe { res.assume_init() };
    assert_eq!(len, size_of::<U>());
    Ok(res)
}

pub trait EcReadmemExt: EcHasReadmem {
    fn ec_read_u8(&self, offset: i32) -> Result<u8> {
        ec_read_any(self, offset)
    }
    fn ec_read_u16(&self, offset: i32) -> Result<u16> {
        ec_read_any(self, offset)
    }
    fn ec_read_u32(&self, offset: i32) -> Result<u32> {
        ec_read_any(self, offset)
    }
}

impl<T: EcHasReadmem> EcReadmemExt for T {}
