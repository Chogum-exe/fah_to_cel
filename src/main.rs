use std::io;

fn main() {
    let mut stop = false;
    loop {
        if stop {
            break;
        };
        let mut unit = String::new();
        println!("Choose measure to convert: fah or cel ex. 'fah'");
        io::stdin().read_line(&mut unit).expect("Cannot read input");
        let mut temp = String::new();
        println!("Enter tempreture value ex. '30'");
        io::stdin().read_line(&mut temp).expect("Cannot read input");
        let temp: i32 = temp.trim().parse().expect("Invalid Temp Input");
        if unit.trim().eq("fah") {
            fah_conv(temp);
        } else if unit.trim().eq("cel") {
            cel_conv(temp);
        } else {
            println!("invalid units");
        }
        println!("Go again? (y/n)");
        let mut res = String::new();
        io::stdin().read_line(&mut res).expect("Can't read input");
        if res.trim().to_lowercase().eq("y") {
            stop = false;
        } else {
            stop = true;
        }
        for _i in 0..50 {
            println!("");
        }
    }
}
fn fah_conv(f: i32) {
    let c = (f - 32) * 5 / 9;
    println!("{} Fah is {} Cel", f, c);
}
fn cel_conv(c: i32) {
    let f = (c * 9 / 5) + 32;
    println!("{} Cel is {} Fah", c, f);
}
