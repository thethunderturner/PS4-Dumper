use crate::scanning::discovery::discover_ps4s;

mod scanning;

fn main() {
    let ps4s = discover_ps4s()
        .expect("Scanning failed!");

    println!("{:#?}", ps4s);
}