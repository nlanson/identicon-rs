use identicon_rs::Identicon;

fn main() {
    let input: String = String::from("identicon.rs example");
    let identicon: Identicon::Identicon = Identicon::Identicon::new(input);
    identicon.render("../identicon");
}