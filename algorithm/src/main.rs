mod factorization;
mod sort;
mod permutation;

static mut RESULT: Vec<String> = Vec::new();

fn main() {
    unsafe {
        println!("{:?}", RESULT);
        RESULT.push("1".to_string());
        println!("{:?}", RESULT);
    }
}
