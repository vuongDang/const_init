mod generated;
use generated::settings::*;

fn main() {
    let present: &str = "I should be present in the binary";
    let absent: &str = "I should be absent in the binary";

    if FOO && BAR == 1 && a::B == [1, 2, -3] && a::C == 3.14 && a::D == "ding!" {
        // Should be kept during compiler optimizations
        println!("{}", present);
    } else {
        // Should be removed by compiler optimizations
        println!("{}", absent);
    }
}
