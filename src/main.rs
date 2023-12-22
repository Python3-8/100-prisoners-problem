use prisoner_problem::simulate;

const NTIMES: usize = 1_000_000;
const NPRISONERS: usize = 100;

fn main() {
    let mut nwins = 0;
    for _i in 0..NTIMES {
        if simulate(NPRISONERS).1 == 0 {
            nwins += 1;
        }
    }
    println!("{nwins} wins out of {NTIMES} iterations with {NPRISONERS} prisoners each time");
    println!("winrate: {}%", (nwins as f64 * 100.) / NTIMES as f64);
}
