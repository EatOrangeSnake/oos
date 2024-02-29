pub type HND = *mut ();


#[link(name = "kernel32")]
extern "stdcall"{
    #[link_name = "CloseHandle"]
    pub fn close(hand: HND) -> crate::def::BOOL;
}


pub mod ascii{
    #[link(name = "kernel32")]
    extern "stdcall"{
        #[link_name = "GetModuleHandleA"]
        pub fn module(hand: crate::def::CSTR) -> super::HND;
    }
}
