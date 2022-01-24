use std::{thread, time};
use webbrowser;

// TODO: add something to open a terminal to update and after to install the debs
/*
    linux-headers-VERSION-NUMBER_all.deb
    linux-headers-VERSION-NUMBER_amd64.deb
    linux-image-VERSION-NUMBER_amd64.deb
    linux-modules-VERSION-NUMBER_amd64.deb
*/

#[allow(unused_must_use)]
fn main() {
    loop {
        // wondering if I can make it download from https://kernel.ubuntu.com/~kernel-ppa/mainline/?C=N;O=D automatically (even if I write it in a stupid way)
        webbrowser::open("https://kernel.ubuntu.com/~kernel-ppa/mainline/?C=N;O=D");
        thread::sleep(time::Duration::from_secs(86400));
    }
}
