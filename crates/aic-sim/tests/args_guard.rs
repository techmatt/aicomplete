//! Not a test: a guard on how the tests were asked for (045). The aliases in
//! `.cargo/config.toml` end in `-- --include-ignored`, so `cargo test-all --
//! --test-threads=2` hands the test binaries a second `--`, past which libtest
//! reads the flag as a test-name filter: every binary then runs nothing and
//! the whole command passes. This binary has no harness, so it sees the same
//! arguments and can refuse them.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let after = args.iter().skip_while(|a| *a != "--").skip(1);
    if let Some(flag) = after.into_iter().find(|a| a.starts_with('-')) {
        eprintln!(
            "error: {flag:?} came after a second `--`, so libtest reads it as a test-name filter and runs \
             nothing. With the aliases, flags go straight after the name: `cargo test-all --test-threads=2`."
        );
        std::process::exit(1);
    }
}
