mod scanning;

fn main() {
    scanning::discovery::discover_ps4s()
        .expect("Scanning failed!");
}