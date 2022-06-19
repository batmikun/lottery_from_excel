mod lib;

use lib::run_lottery;

fn main() {
    let path: String = format!("{}/rifa.xlsx", env!("CARGO_MANIFEST_DIR"));

    run_lottery(path, 3);
}
