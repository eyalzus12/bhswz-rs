mod bhswz;
use bhswz::SwzRandom;

fn main() {
    let mut random = SwzRandom::new(0);
    for _ in 0..100 {
        println!("{}", random.next());
    }
}
