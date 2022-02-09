use std::{io::Read, process::Command, thread, time};
use webbrowser;

// TODO: add something to open a terminal to update and after to install the lowlatency debs
/*
    linux-headers-VERSION-NUMBER_all.deb
    linux-headers-VERSION-NUMBER_amd64.deb
    linux-image-VERSION-NUMBER_amd64.deb
    linux-modules-VERSION-NUMBER_amd64.deb
*/

#[allow(unused_must_use)]
fn main() {
    loop {
        // wondering if I can make it download from https://kernel.ubuntu.com/ automatically (even if I write it in a stupid way)
        Command::new("bash")
            .arg("./getkernel")
            .spawn()
            .expect("failed step 0");
        let mut kernver = std::fs::File::open("/tmp/kernver-latest").unwrap();
        let mut contents = String::new();
        kernver.read_to_string(&mut contents).unwrap();
        webbrowser::open(&*("https://kernel.ubuntu.com/~kernel-ppa/mainline/v".to_owned() + &contents));
        thread::sleep(time::Duration::from_secs(5));
        // thread::sleep(time::Duration::from_secs(86400));
    }
}
