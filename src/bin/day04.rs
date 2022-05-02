extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let input = "bgvyzdsv".as_bytes();
    let mut hasher = Md5::new();
    for i in (0..std::u64::MAX) {
        hasher.input(input);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            println!("{}", i);
            break;
        }
        hasher.reset();
    }
}
