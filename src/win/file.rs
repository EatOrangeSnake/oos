use crate::def::*;


pub mod def{
    pub type FILE = *mut ();
    pub mod access{
        use crate::def::DWORD;
        pub const ALL: DWORD = 0x10000000;
        pub const EXE: DWORD = 0x20000000;
        pub const WRITE: DWORD = 0x40000000;
        pub const READ: DWORD = 0x80000000;
    }
    pub mod share{
        use crate::def::DWORD;
        pub const NONE: DWORD = 0x0;
        pub const DEL: DWORD = 0x4;
        pub const READ: DWORD = 0x1;
        pub const WRITE: DWORD = 0x2;
    }
    pub mod creation{
        use crate::def::DWORD;
        pub const TRUNCATE: DWORD = 0x5;
        pub mod new{
            use crate::def::DWORD;
            pub const NEW: DWORD = 0x1;
            pub const ALWAYS: DWORD = 0x2;
        }
        pub mod open{
            use crate::def::DWORD;
            pub const EXIST: DWORD = 0x3;
            pub const ALWAYS: DWORD = 0x4;
        }
    }
    pub mod attr{
        use crate::def::DWORD;
        pub const NORMAL: DWORD = 0x80;
    }
    pub mod offset{
        use crate::def::DWORD;
        pub const BEGIN: DWORD = 0x0;
        pub const END: DWORD = 0x1;
        pub const CUR: DWORD = 0x2;
    }
}


use def::*;


pub mod ascii{
    use crate::def::*;
    use super::def::*;
    #[link(name = "kernel32")]
    extern "stdcall"{
        #[link_name = "CreateFileA"]
        pub fn get(
            path: CSTR, 
            access: DWORD, 
            share: DWORD, 
            security: VOID, 
            creation: DWORD, 
            flag_attr: DWORD, 
            tmpfile: FILE
        ) -> FILE;
        #[link_name = "DeleteFileA"]
        pub fn del(
            path: CSTR
        ) -> BOOL;
        #[link_name = "CreateDirectoryA"]
        pub fn dir(
            path: CSTR, 
            security: VOID
        ) -> BOOL;
    }
}


pub mod unicode{
    use crate::def::*;
    use super::def::*;
    #[link(name = "kernel32")]
    extern "stdcall"{
        #[link_name = "CreateFileW"]
        pub fn get(
            path: CWSTR, 
            access: DWORD, 
            share: DWORD, 
            security: VOID, 
            creation: DWORD, 
            flag_attr: DWORD, 
            tmpfile: FILE
        ) -> FILE;
        #[link_name = "DeleteFileW"]
        pub fn del(
            path: CWSTR
        ) -> BOOL;
        #[link_name = "CreateDirectoryW"]
        pub fn dir(
            path: CWSTR, 
            security: VOID
        ) -> BOOL;
    }
}


#[link(name = "kernel32")]
extern "stdcall"{
    #[link_name = "WriteFile"]
    pub fn write(
        hand: FILE, 
        buffer: CSTR, 
        n: DWORD, 
        receive: *mut DWORD, 
        overlap: VOID
    ) -> BOOL;
    #[link_name = "ReadFile"]
    pub fn read(
        hand: FILE, 
        buffer: CSTR, 
        n: DWORD, 
        receive: *mut DWORD, 
        overlap: VOID
    ) -> BOOL;
    #[link_name = "GetFileSize"]
    pub fn size(
        hand: FILE, 
        high: *mut DWORD
    ) -> DWORD;
    #[link_name = "SetFilePointer"]
    pub fn setp(
        hand: FILE, 
        low: DWORD, 
        high: *mut DWORD, 
        offset: DWORD
    ) -> DWORD;
    #[link_name = "SetEndOfFile"]
    pub fn seof(
        hand: FILE
    ) -> BOOL;
}
