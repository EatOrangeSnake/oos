#[repr(C)]
#[derive(Debug)]
pub struct MSG{
    pub wnd: crate::win::hand::HND, 
    pub msg: crate::def::UINT, 
    pub wp: crate::def::DWORD, 
    pub lp: crate::def::DWORD, 
    pub time: crate::def::DWORD, 
    pub pnt: super::pnt::PNT, 
    pub private: crate::def::DWORD
}


pub mod ascii{
    #[link(name = "user32")]
    extern "stdcall"{
        #[link_name = "GetMessageA"]
        pub fn get(
            msg: *mut super::MSG, 
            hwnd: crate::win::hand::HND, 
            fil_min: crate::def::UINT, 
            fil_max: crate::def::UINT
        ) -> crate::def::BOOL;
        #[link_name = "DispatchMessageA"]
        pub fn dispatch(
            msg: *mut super::MSG
        ) -> crate::def::BOOL;
    }
}


pub mod unicode{
    #[link(name = "user32")]
    extern "stdcall"{
        #[link_name = "GetMessageW"]
        pub fn get(
            msg: *mut super::MSG, 
            hwnd: crate::win::hand::HND, 
            fil_min: crate::def::UINT, 
            fil_max: crate::def::UINT
        ) -> crate::def::BOOL;
        #[link_name = "DispatchMessageW"]
        pub fn dispatch(
            msg: *mut super::MSG
        ) -> crate::def::BOOL;
    }
}


#[link(name = "user32")]
extern "stdcall"{
    #[link_name = "TranslateMessage"]
    pub fn translate(
        msg: *mut MSG
    ) -> crate::def::BOOL;
}
