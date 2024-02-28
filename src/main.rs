use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let known_hosts = read_known_hosts().unwrap_or_else(|err| {
        panic!(
            "Reading known hosts file at {} failed.\n{}",
            known_hosts_path().to_str().unwrap(),
            err
        );
    });

    let mut new_known_hosts_vec: Vec<String> = Vec::new();

    known_hosts.split("\n").into_iter().for_each(|x| {
        let a = x.split(" ").collect::<Vec<&str>>();
        let hostname = a[0];

        if hostname != args[1] {
            new_known_hosts_vec.push(x.to_string());
        } else {
            println!("Removed {} from known_hosts.", hostname)
        }
    });

    let new_known_hosts = new_known_hosts_vec.join("\n");

    if new_known_hosts == read_known_hosts().unwrap() {
        println!("{} not found in known_hosts.", args[1]);
    }

    write_known_hosts(new_known_hosts).unwrap_or_else(|err| {
        panic!(
            "Writing known hosts file at {} failed.\n{}",
            known_hosts_path().to_str().unwrap(),
            err
        );
    })
}

fn known_hosts_path() -> PathBuf {
    let home_dir = env!("HOME");

    PathBuf::from(home_dir).join(".ssh/known_hosts")
}

fn read_known_hosts() -> Result<String, std::io::Error> {
    fs::read_to_string(known_hosts_path())
}

fn write_known_hosts(contents: String) -> Result<(), std::io::Error> {
    fs::write(known_hosts_path(), contents)
}
