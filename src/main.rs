use std::env;

mod download;

#[allow(unused_must_use)]
fn main() {
    let mut args: Vec<String> = env::args().collect();
    // Remove the first argument
    args.remove(0);

    if args.len() < 2 {
        return println!("You need to provide a download url and a file name in that order")
    }

    // Actually run the download function
    download::download(args);
}
