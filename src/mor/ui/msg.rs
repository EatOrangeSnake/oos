use crate::com::ui;
use crate::com::hand::HND;


pub struct Msg{
    pub msg: ui::msg::MSG
}


impl From<&super::win::Win> for Msg{
    #[inline]
    fn from(wnd: &super::win::Win) -> Msg{
        Msg { msg: ui::msg::MSG{
            wnd: unsafe{*(wnd as *const super::win::Win as *mut HND)}, 
            msg: 0, 
            wp: 0, 
            lp: 0, 
            time: 0, 
            pnt: ui::pnt::PNT{
                x: 0, y: 0
            }, 
            private: 0
        } }
    }
}
impl Msg{
    #[inline]
    pub fn get(&self) -> crate::def::BOOL{
        unsafe{ui::msg::ascii::get(
            self as *const Msg as *mut ui::msg::MSG, 
            *(self as *const Msg as *mut HND), 
            0, 0
        )}
    }
    #[inline]
    pub fn tran(&self){
        unsafe{ui::msg::translate(
            self as *const Msg as *mut ui::msg::MSG
        )};
    }
    #[inline]
    pub fn dispatch(&self){
        unsafe{
            ui::msg::ascii::dispatch(self as *const Msg as *mut ui::msg::MSG)
        };
    }
}
