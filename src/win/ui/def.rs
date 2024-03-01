pub const DEFAULT: crate::def::INT = 0x80000000u32 as crate::def::INT;
pub mod style{
    use crate::def::DWORD;
    pub const OVERLAP: DWORD = 0x0;
    pub const BORDER: DWORD = 0x800000;
    pub const CAPTION: DWORD = 0xc00000;
    pub const CHILD: DWORD = 0x40000000;
    pub const MAXIZABLE: DWORD = 0x10000;
    pub const MINIZABLE: DWORD = 0x20000;
    pub const THICK: DWORD = 0x40000;
    pub const MENU: DWORD = 0x80000;
    pub const WINDOW: DWORD = BORDER | CAPTION | MAXIZABLE | MINIZABLE | THICK | 
    OVERLAP | MENU;
    pub const VISIBLE: DWORD = 0x10000000;
}
pub mod msg{
    use crate::def::DWORD;
    pub const NULL: DWORD = 0;
    pub const CREATE: DWORD = 1;
    pub const DESTORY: DWORD = 2;
    pub const MOVE: DWORD = 3;
    pub const QUIT: DWORD = 12;
    pub const PAINT: DWORD = 15;
}
