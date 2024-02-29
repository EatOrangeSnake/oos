pub type VOID = *mut ();
pub const NULL: VOID = 0usize as VOID;


pub type BYTE = u8;
pub type WORD = u16;
pub type DWORD = u32;
pub type QWORD = u64;


pub type INT = i32;
pub type UINT = u32;
pub type INT32 = i32;
pub type INT64 = i64;
pub type BOOL = u32;


pub type CHAR = u8;
pub type WCHAR = u16;
pub type CSTR = *const CHAR;
pub type STR = *mut CHAR;
pub type CWSTR = *const WCHAR;
pub type WSTR = *mut WCHAR;


pub const SNULL: CSTR = NULL as CSTR;
pub const SWNULL: CWSTR = NULL as CWSTR;


pub macro stringify($_str: expr){
    $_str as *const str as *const u8
}


pub macro voidify($_dat: expr){
    $_dat as *mut ()
}


pub macro nullify($_type: ty){
    0usize as $_type
}


pub macro constify($_dat: expr, $_type: ty){
    $_dat as *const $_type
}


pub macro mutify($_dat: expr, $_type: ty){
    $_dat as *const $_type as *mut $_type
}


pub macro default($_type: ty){
    unsafe{std::mem::transmute::<[u8;std::mem::size_of::<$_type>()], $_type>(
        [0u8;std::mem::size_of::<$_type>()])}
}
