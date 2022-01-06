use permutations::n_choose_m;

fn main() {
    for n in 0..=10 {
        for m in 0..=10 {
            print!("{:>3}, ", n_choose_m(n, m).len());
        }
        println!();
    }
}
