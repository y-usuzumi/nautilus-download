use std::io::Error as IOError;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::path::Path;
use hyper::Error as HyperError;
use hyper::client::Client;

#[derive(Debug)]
pub enum DownloadError {
    RequestError,
    FileError
}

impl From<HyperError> for DownloadError {
    fn from(err: HyperError) -> DownloadError {
        DownloadError::RequestError
    }
}

impl From<IOError> for DownloadError {
    fn from(err: IOError) -> DownloadError {
        DownloadError::FileError
    }
}

pub fn download(url: &String, save_to: &Path) -> Result<usize, DownloadError> {
    let client = Client::new();
    let mut resp = try!(client.get(url).send());
    let mut file = try!(OpenOptions::new().write(true).create(true).open(save_to));
    let mut buf = [0; 4096];
    let mut total_size = 0;
    loop {
        let read_size = try!(resp.read(&mut buf));
        if read_size > 0 {
            let write_size = try!(file.write(&buf[..read_size]));
            assert!(read_size == write_size);
            total_size += read_size;
        } else {
            let _ = try!(file.sync_all());
            break;
        }
    }
    Ok(total_size)
}
