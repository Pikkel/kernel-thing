use std::{fs, io::Read, io::Write, process::Command, thread, time};
use webbrowser;

// TODO: remove the webbrowser part and make it download and hopefully install the pkgs

#[allow(unused_must_use)]
fn main() {
    fs::create_dir_all("/tmp/.kernver/");
    loop {
        // curl -s https://kernel.ubuntu.com/\~kernel-ppa/mainline/v5.16.8/amd64/ | grep -A1 'href=' | grep -oP '(?<=deb">).*(?=lowlatency).*(?=</a.*)'
        // curl -s https://kernel.ubuntu.com/\~kernel-ppa/mainline/v5.16.8/amd64/ | grep -A1 'href=' | grep -oP '(?<=all.deb">).*(?=</a.*)'
        Command::new("bash")
            // command from here >> https://stackoverflow.com/questions/25078065/how-to-check-latest-kernel-version-by-bash
            .arg("./getkernel")
            .spawn()
            .expect("failed step 0");
        let mut kernver = std::fs::File::open("/tmp/.kernver/kern-latest").unwrap();
        let mut ver = String::new();
        kernver.read_to_string(&mut ver).unwrap();
        // TODO: make the commands not go into 4 lines
        let mut file =
            std::fs::File::create("/tmp/.kernver/downloadkern").expect("failed to create");
        file.write_all(("curl -s https://kernel.ubuntu.com/\\~kernel-ppa/mainline/v".to_owned() + &ver + "/amd64/ | grep -A1 'href=' | grep -oP '(?<=deb\">).*(?=lowlatency).*(?=</a.*)'\n").as_bytes())
            .expect("failed to write config");
        file.write_all(
            ("curl -s https://kernel.ubuntu.com/\\~kernel-ppa/mainline/v".to_owned()
                + &ver
                + "/amd64/ | grep -A1 'href=' | grep -oP '(?<=all.deb\">).*(?=</a.*)'")
                .as_bytes(),
        )
        .expect("failed to write config");
        webbrowser::open(&*("https://kernel.ubuntu.com/~kernel-ppa/mainline/v".to_owned() + &ver));
        thread::sleep(time::Duration::from_secs(5));
        // thread::sleep(time::Duration::from_secs(86400));
    }
}
