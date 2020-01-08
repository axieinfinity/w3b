use std::{fs::File, io, io::Write, path::PathBuf};

const BASE_PATH: &'static str = env!("CARGO_MANIFEST_DIR");
const PATH: &'static str = "src/bytes/bytes.rs";

fn main() -> io::Result<()> {
    let path = PathBuf::from(BASE_PATH).join(PATH);
    let mut file = File::create(path).unwrap();

    writeln!(file, "use crate::impl_bytes;")?;
    writeln!(file)?;

    for size in 1..=32 {
        writeln!(file, "impl_bytes!(Bytes{0}; size = {0});", size)?;
    }

    Ok(())
}
