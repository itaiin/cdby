use regex::Regex;
use std::ffi::CString;
use nix::unistd::execv;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    root: String,

    #[arg()]
    pattern: String,

    #[arg(long, default_value = "false")]
    dir_parent: bool,
}

fn to_cstr(s: &str) -> CString {
    CString::new(s.as_bytes()).unwrap()
}

#[allow(unreachable_code)]
fn do_it(args: Args) {
    let re = Regex::new(args.pattern.as_str()).unwrap();
    for entry in ignore::Walk::new(args.root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        if re.is_match(path_str) {
            println!("{}", path_str);
            let jump_to = if path.is_file() || path.is_symlink() || args.dir_parent {
                path.parent().unwrap()
            } else if path.is_dir() {
                path
            } else {
                panic!("Unsupported file type");
            };
            std::env::set_current_dir(jump_to).unwrap();
            execv(to_cstr("/bin/bash").as_c_str(), &[to_cstr("bash")]).unwrap();
        }
    }
    println!("Could not find a match");
    std::process::exit(1);
}

fn main() {
    let args = Args::parse();
    do_it(args);
}
