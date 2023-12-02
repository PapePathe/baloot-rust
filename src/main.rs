use belote::cards::Card;
use belote::cards::CardColor;
use belote::cards::CardFamily;


fn main() {
    let c: Card = Card::new(CardColor::DIAMONDS, CardFamily::ACE);

    println!("{:#?}", c);

}
