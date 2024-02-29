pub use crate::com::file;
use crate::com::hand::{HND, close};


pub static mut TEMP: crate::def::DWORD = 0u32;


#[derive(Debug, Clone)]
pub struct File{
    pub file: HND
}


impl From<&str> for File{
    #[inline]
    fn from(value: &str) -> Self {
        File { file: unsafe{file::ascii::get(
            crate::def::stringify!(value), 
            file::def::access::ALL, 
            file::def::share::NONE, 
            crate::def::NULL, 
            file::def::creation::new::ALWAYS, 
            file::def::attr::NORMAL, 
            crate::def::NULL
        )} }
    }
}


impl Drop for File{
    #[inline]
    fn drop(&mut self) {
        unsafe{close(self.file)};
    }
}


impl<const L: usize> std::ops::ShlAssign<&[u8;L]> for File{
    #[inline]
    fn shl_assign(&mut self, rhs: &[u8;L]) {
        unsafe{file::write(
            self.file, 
            rhs as *const [u8;L] as *mut u8, 
            L as crate::def::DWORD, 
            std::ptr::addr_of_mut!(TEMP), 
            crate::def::NULL
        )};
    }
}


impl<const L: usize> std::ops::Shl<&[u8;L]> for File{
    type Output = Self;
    #[inline]
    fn shl(mut self, rhs: &[u8;L]) -> Self::Output {
        self <<= rhs;
        self
    }
}


impl<const L: usize> std::ops::ShrAssign<&[u8;L]> for File{
    #[inline]
    fn shr_assign(&mut self, rhs: &[u8;L]) {
        unsafe{file::read(
            self.file, 
            rhs as *const [u8;L] as *mut u8, 
            L as crate::def::DWORD, 
            std::ptr::addr_of_mut!(TEMP), 
            crate::def::NULL
        )};
    }
}


impl<const L: usize> std::ops::Shr<&[u8;L]> for File{
    type Output = Self;
    #[inline]
    fn shr(mut self, rhs: &[u8;L]) -> Self::Output {
        self >>= rhs;
        self
    }
}


impl std::ops::SubAssign<crate::def::DWORD> for File{
    #[inline]
    fn sub_assign(&mut self, rhs: crate::def::DWORD) {
        unsafe{file::setp(
            self.file, 
            rhs, 
            std::ptr::addr_of_mut!(TEMP), 
            file::def::offset::END
        )};
    }
}


impl std::ops::AddAssign<crate::def::DWORD> for File{
    #[inline]
    fn add_assign(&mut self, rhs: crate::def::DWORD) {
        unsafe{file::setp(
            self.file, 
            rhs, 
            std::ptr::addr_of_mut!(TEMP), 
            file::def::offset::BEGIN
        )};
    }
}


impl std::ops::BitOrAssign<crate::def::DWORD> for File{
    #[inline]
    fn bitor_assign(&mut self, rhs: crate::def::DWORD) {
        unsafe{file::setp(
            self.file, 
            rhs, 
            std::ptr::addr_of_mut!(TEMP), 
            file::def::offset::CUR
        )};
    }
}


impl std::ops::Neg for File{
    type Output = ();
    #[inline]
    fn neg(self) -> Self::Output {
        unsafe{file::seof(self.file)};
    }
}
