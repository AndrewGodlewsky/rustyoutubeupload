use std::process::{Child, Command};

pub fn install_requirements() {
    // pip install -r requirements.txt
    let output = Command::new("pip")
        .arg("install")
        .arg("-r")
        .arg("requirements.txt")
        .output()
        .expect("`pip install -r requirements.txt` not working");

    println!("{}", String::from_utf8(output.stdout).unwrap());
}

pub fn run() {
    let output = Command::new("python")
        .arg("main.py")
        .output()
        .expect("`python main.py` not working");

    println!("{}", String::from_utf8(output.stdout).unwrap());
}
