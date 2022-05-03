extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let input = "bgvyzdsv".as_bytes();
    let mut hasher = Md5::new();
    for i in 0..std::u64::MAX {
        hasher.input(input);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let s = String::from_iter(output.iter().map(|b| format!("{:02X}", b)));
        if s.starts_with("000000") {
            println!("{}", i);
            break;
        }
        hasher.reset();
    }
}
