fn env(s: &str) -> String { std::env::var(s).unwrap() }

fn main() {
    for s in env("DEP_MINRT_LINK_SEARCH").split(':') {
        println!("cargo:rustc-link-search={}", s);
    }

    for s in env("DEP_MINRT_LINK_ARGS").split(':') {
        println!("cargo:rustc-link-arg-bins={}", s);
    }

    for s in env("DEP_MINRT_LINK_LIBS").split(':') {
        println!("cargo:rustc-link-lib=static={}", s);
    }
}
