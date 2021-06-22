use identicon_rs::Identicon;

fn main() {

    //identicon_from_string();
    identicon_from_readln();

}

fn identicon_from_string() {
    //Create a predefined identicon from a string.
    let input: String = String::from("test431");
    let identicon: Identicon::Identicon = Identicon::Identicon::new(input);
    identicon.render("../identicon");
}

fn identicon_from_readln() {
    //Create identicon from user input.
    let identicon: Identicon::Identicon = Identicon::Identicon::new_readln();
    identicon.render("../identicon_from_readln");
}