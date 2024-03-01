use crate::com::ui;
use crate::com::hand::HND;


pub struct Win{
    pub win: HND
}


impl Win{
    #[inline]
    pub fn base_tc(title: &str, class: &str) -> Self{
        Self { 
            win: unsafe{ui::win::ascii::new(
                0, 
                crate::def::stringify!(class), 
                crate::def::stringify!(title), 
                ui::def::style::WINDOW, 
                ui::def::DEFAULT, 
                ui::def::DEFAULT, 
                ui::def::DEFAULT, 
                ui::def::DEFAULT, 
                crate::def::NULL, 
                crate::def::NULL, 
                crate::def::NULL, 
                crate::def::NULL
            )} 
        }
    }
    #[inline]
    pub fn base_tcs(title: &str, class: &str, w: crate::def::INT, 
    h: crate::def::INT) -> Self{
        Self { 
            win: unsafe{ui::win::ascii::new(
                0, 
                crate::def::stringify!(class), 
                crate::def::stringify!(title), 
                ui::def::style::WINDOW, 
                ui::def::DEFAULT, 
                ui::def::DEFAULT, 
                w, 
                h, 
                crate::def::NULL, 
                crate::def::NULL, 
                crate::def::NULL, 
                crate::def::NULL
            )} 
        }
    }
    #[inline]
    pub fn base_tcss(title: &str, class: &str, w: crate::def::INT, 
    h: crate::def::INT, style: crate::def::DWORD) -> Self{
        Self { 
            win: unsafe{ui::win::ascii::new(
                0, 
                crate::def::stringify!(class), 
                crate::def::stringify!(title), 
                style, 
                ui::def::DEFAULT, 
                ui::def::DEFAULT, 
                w, 
                h, 
                crate::def::NULL, 
                crate::def::NULL, 
                crate::def::NULL, 
                crate::def::NULL
            )} 
        }
    }
    #[inline]
    pub fn child_c(&self, class: &str) -> Self{
        Self {
            win: unsafe{
                ui::win::ascii::new(
                    0, 
                    crate::def::stringify!(class), 
                    crate::def::SNULL, 
                    ui::def::style::CHILD, 
                    ui::def::DEFAULT, 
                    ui::def::DEFAULT, 
                    ui::def::DEFAULT, 
                    ui::def::DEFAULT, 
                    *(self as *const Win as *mut HND), 
                    crate::def::NULL, 
                    crate::def::NULL, 
                    crate::def::NULL
                )
            }
        }
    }
    #[inline]
    pub fn child_tc(&self, title: &str, class: &str) -> Self{
        Self {
            win: unsafe{
                ui::win::ascii::new(
                    0, 
                    crate::def::stringify!(class), 
                    crate::def::stringify!(title), 
                    ui::def::style::CHILD, 
                    ui::def::DEFAULT, 
                    ui::def::DEFAULT, 
                    ui::def::DEFAULT, 
                    ui::def::DEFAULT, 
                    *(self as *const Win as *mut HND), 
                    crate::def::NULL, 
                    crate::def::NULL, 
                    crate::def::NULL
                )
            }
        }
    }
    #[inline]
    pub fn child_tcp(&self, title: &str, class: &str, 
    x: crate::def::INT, y: crate::def::INT) -> Self{
        Self {
            win: unsafe{
                ui::win::ascii::new(
                    0, 
                    crate::def::stringify!(class), 
                    crate::def::stringify!(title), 
                    ui::def::style::CHILD, 
                    x, 
                    y, 
                    ui::def::DEFAULT, 
                    ui::def::DEFAULT, 
                    *(self as *const Win as *mut HND), 
                    crate::def::NULL, 
                    crate::def::NULL, 
                    crate::def::NULL
                )
            }
        }
    }
    #[inline]
    pub fn show(&self){
        unsafe{ui::win::show(*(self as *const Win as *mut HND), 1)};
    }
}
