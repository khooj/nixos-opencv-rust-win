use std::env;

fn main() {
    let pthreads_win = env::var("WIN_PTHREADS").unwrap();
    println!("cargo:rustc-link-search=native={}", pthreads_win);
}
