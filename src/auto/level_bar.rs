// This file was generated by gir (b798f4f) from gir-files (11e0e6d)
// DO NOT EDIT

#[cfg(feature = "v3_6")]
use LevelBarMode;
use Orientable;
use Widget;
use ffi;
#[cfg(feature = "v3_6")]
use ffi::GtkLevelBar;
use glib::object::Downcast;
#[cfg(feature = "v3_6")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_6")]
use glib_ffi::gpointer;
#[cfg(feature = "v3_6")]
use libc::c_char;
#[cfg(feature = "v3_6")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_6")]
use std::mem;
#[cfg(feature = "v3_6")]
use std::mem::transmute;

glib_wrapper! {
    pub struct LevelBar(Object<ffi::GtkLevelBar>): Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_level_bar_get_type(),
    }
}

impl LevelBar {
    #[cfg(feature = "v3_6")]
    pub fn new() -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn new_for_interval(min_value: f64, max_value: f64) -> LevelBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_level_bar_new_for_interval(min_value, max_value)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn add_offset_value(&self, name: &str, value: f64) {
        unsafe {
            ffi::gtk_level_bar_add_offset_value(self.to_glib_none().0, name.to_glib_none().0, value);
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_inverted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_level_bar_get_inverted(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_max_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_max_value(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_min_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_min_value(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_mode(&self) -> LevelBarMode {
        unsafe {
            from_glib(ffi::gtk_level_bar_get_mode(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_offset_value(&self, name: Option<&str>) -> Option<f64> {
        unsafe {
            let mut value = mem::uninitialized();
            let ret = from_glib(ffi::gtk_level_bar_get_offset_value(self.to_glib_none().0, name.to_glib_none().0, &mut value));
            if ret { Some(value) } else { None }
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_level_bar_get_value(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn remove_offset_value(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_level_bar_remove_offset_value(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            ffi::gtk_level_bar_set_inverted(self.to_glib_none().0, inverted.to_glib());
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn set_max_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_max_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn set_min_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_min_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn set_mode(&self, mode: LevelBarMode) {
        unsafe {
            ffi::gtk_level_bar_set_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_level_bar_set_value(self.to_glib_none().0, value);
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn connect_offset_changed<F: Fn(&LevelBar, &str) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&LevelBar, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "offset-changed",
                transmute(offset_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_6")]
unsafe extern "C" fn offset_changed_trampoline(this: *mut GtkLevelBar, name: *mut c_char, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&LevelBar, &str) + 'static> = transmute(f);
    f(&from_glib_none(this), &String::from_glib_none(name))
}
