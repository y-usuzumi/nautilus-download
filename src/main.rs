extern crate nautilus_download;

use std::env;
use std::path::Path;
use nautilus_download::download::download;

fn main() {
    let args: Vec<_> = env::args().collect();
    let url = &args[1];
    let save_to = &args[2];
    match download(url, Path::new(save_to)) {
        Ok(size) => println!("Done. Bytes written: {}", size),
        Err(err) => println!("Error: {:?}", err)
    }
}
