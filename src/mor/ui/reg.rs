use crate::com::ui;


pub struct A{
    pub cls: ui::reg::WCLSA
}


impl A{
    #[inline]
    pub fn from_cp(class: &str, proc: ui::reg::PROC) -> A{
        Self{
            cls: ui::reg::WCLSA{
                style: 0, 
                winproc: proc as crate::def::VOID, 
                cls_extra: 0, 
                wnd_extra: 0, 
                inst: crate::def::NULL, 
                icon: crate::def::NULL, 
                cursor: crate::def::NULL, 
                brush: crate::def::NULL, 
                menu: crate::def::SNULL, 
                cls: crate::def::stringify!(class)
            }
        }
    }
    #[inline]
    pub fn reg(&self){
        unsafe{ui::reg::ascii::reg(self as *const Self as *const ui::reg::WCLSA)};
    }
}
