extern crate hyper;

extern crate gdk;
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk;
extern crate gtk_sys as gtk_ffi;
extern crate libc;
#[macro_use(nautilus_module, nautilus_menu_item_activate_cb)]
extern crate nautilus_extension;

pub mod download;
pub mod nautilus_download;
mod menu_provider;
mod gtk_helpers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
