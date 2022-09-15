fn main() {
    let r = catr::get_args();
    let err = r.and_then(catr::run);
    if let Err(e) = err {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
