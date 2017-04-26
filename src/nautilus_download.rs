use glib_ffi::GType;
use gobject_ffi::GTypeModule;
use libc::c_int;
use nautilus_extension::NautilusModule;

use menu_provider;

nautilus_module!(init);

fn init(module: *mut GTypeModule) -> GType {
    println!("Initializing Nautilus Download");

    NautilusModule::new(module, "NautilusDownloadExtension")
        .add_menu_provider(menu_provider::NautilusDownloadMenuProvider {})
        .register()
}
