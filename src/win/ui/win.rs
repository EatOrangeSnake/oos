pub mod ascii{
    use crate::win::hand::HND;
    use crate::def::*;
    #[link(name = "user32")]
    extern "stdcall"{
        #[link_name = "CreateWindowExA"]
        pub fn new(
            dw_style: DWORD, 
            class: CSTR, 
            title: CSTR, 
            style: DWORD, 
            x: INT, 
            y: INT, 
            w: INT, 
            h: INT, 
            parent: HND, 
            menu: HND, 
            inst: HND, 
            param: VOID
        ) -> HND;
    }
}


#[link(name = "user32")]
extern "stdcall"{
    #[link_name = "ShowWindow"]
    pub fn show(
        hnd: crate::win::hand::HND, 
        show: crate::def::INT
    ) -> crate::def::BOOL;
}
