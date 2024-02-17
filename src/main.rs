use bip39::Mnemonic;
use itertools::Itertools;

fn main() {
    let words = 24;
    let mnemonic = Mnemonic::generate(words).unwrap();
    println!("{}", mnemonic.word_iter().join(" "));
}
