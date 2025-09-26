use settings_as_constants::*;

fn main() {
    println!("foo: {FOO}");
    println!("bar: {BAR}");

    let present = "I should be present in the binary";
    let absent = "I should be absent in the binary";
    if FOO {
        println!("{}", present);
    } else {
        println!("{}", absent);
    }
    if BAR == 1 {
        println!("{}", present);
    } else {
        println!("{}", absent);
    }
}
