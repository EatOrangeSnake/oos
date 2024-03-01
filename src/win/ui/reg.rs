pub type PROC = extern "stdcall" fn(
    crate::com::hand::HND, 
    crate::def::UINT, 
    crate::def::UINT, 
    crate::def::UINT
) -> crate::def::BOOL;


#[repr(C)]
pub struct WCLSA{
    pub style: crate::def::DWORD, 
    pub proc: PROC, 
    pub cls_extra: crate::def::INT, 
    pub wnd_extra: crate::def::INT, 
    pub inst: crate::win::hand::HND, 
    pub icon: crate::win::hand::HND, 
    pub cursor: crate::win::hand::HND, 
    pub brush: crate::win::hand::HND, 
    pub menu: crate::def::CSTR, 
    pub cls: crate::def::CSTR
}


unsafe impl Sync for WCLSA{}


#[repr(C)]
pub struct WCLSEA{
    pub cbsize: crate::def::DWORD, 
    pub style: crate::def::DWORD, 
    pub proc: PROC, 
    pub cls_extra: crate::def::INT, 
    pub wnd_extra: crate::def::INT, 
    pub inst: crate::win::hand::HND, 
    pub icon: crate::win::hand::HND, 
    pub cursor: crate::win::hand::HND, 
    pub brush: crate::win::hand::HND, 
    pub menu: crate::def::CSTR, 
    pub cls: crate::def::CSTR, 
    pub simicon: crate::win::hand::HND
}


unsafe impl Sync for WCLSEA{}


pub mod ascii{
    #[link(name = "user32")]
    extern "stdcall"{
        #[link_name = "RegisterClassA"]
        pub fn reg(
            cls: *const super::WCLSA
        ) -> crate::def::BOOL;
        #[link_name = "RegisterClassExA"]
        pub fn erg(
            cls: *const super::WCLSEA
        ) -> crate::def::BOOL;
    }
}
