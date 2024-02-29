pub mod utype{
    use crate::def::DWORD;
    pub const TRY: DWORD = 0x2;
    pub const CONTINUE: DWORD = 0x6;
    pub const HELP: DWORD = 0x4000;
    pub const OK: DWORD = 0x0;
    pub const IS: DWORD = 0x1;
    pub const AGAIN: DWORD = 0x5;
    pub const YES: DWORD = 0x4;
    pub const KNOW: DWORD = 0x3;
}


pub mod ascii{
    #[link(name = "user32")]
    extern "stdcall"{
        #[link_name = "MessageBoxA"]
        pub fn message(
            wnd: super::super::super::hand::HND, 
            text: crate::def::CSTR, 
            title: crate::def::CSTR, 
            utype: crate::def::UINT
        ) -> crate::def::INT;
    }
}


pub mod unicode{
    #[link(name = "user32")]
    extern "stdcall"{
        #[link_name = "MessageBoxW"]
        pub fn message(
            wnd: super::super::super::hand::HND, 
            text: crate::def::CWSTR, 
            title: crate::def::CWSTR, 
            utype: crate::def::UINT
        ) -> crate::def::INT;
    }
}
