use clap::{self, arg};
use clap_mangen::Man;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::{env, fs};

fn main() -> io::Result<()> {
    let out_dir = {
        let out_dir_var = env::var_os("OUT_DIR").ok_or(ErrorKind::NotFound)?;
        PathBuf::from(out_dir_var)
    };

    let cmd = clap::Command::new("meread")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .args([
            arg!([PATH] "Path to markdown file or directory containing README.md [default: .]"),
            arg!(-e --"export-dir" <EXPORT_DIR> "If supplied, will export the markdown file to HTML in the specified directory"),
            arg!(-f --force "Whether to overwrite the export directory if it exists"),
            arg!(-a --address <ADDRESS> "Address to bind the server to [default: localhost:3000]"),
            arg!(-o --open "Whether to open the browser on serve"),
            arg!(-l --light "Render page in light-mode style"),
        ]);

    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    // the use of $PROFILE isn't recommended, according to
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    if option_env!("PROFILE") != Some("release") {
        // local man page (./meread.1)
        let local_man = {
            let cwd = env::current_dir()?;
            cwd.join("meread.1")
        };
        fs::write(local_man, &buffer)?;
    }

    // final man page
    fs::write(out_dir.join("meread.1"), buffer)?;

    Ok(())
}
