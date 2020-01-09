extern crate autocfg;

fn main() {
    let cfg = autocfg::new();
    cfg.emit_has_type("i128");
}
