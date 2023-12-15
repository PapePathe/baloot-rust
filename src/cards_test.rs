use crate::cards::*;
use crate::gametake::GameTake;
use crate::gametake::Take;

#[test]
fn test_evaluate_for_take_cent() {
    let c = Card::new(CardColor::SPADES, CardFamily::ACE);
    assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 11);

    let c = Card::new(CardColor::SPADES, CardFamily::TEN);
    assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 10);

    let c = Card::new(CardColor::SPADES, CardFamily::KING);
    assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 4);

    let c = Card::new(CardColor::SPADES, CardFamily::QUEEN);
    assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 3);

    let c = Card::new(CardColor::SPADES, CardFamily::JACK);
    assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 2);

    let c = Card::new(CardColor::SPADES, CardFamily::NINE);
    assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 0);

    let c = Card::new(CardColor::SPADES, CardFamily::HEIGHT);
    assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 0);

    let c = Card::new(CardColor::SPADES, CardFamily::SEVEN);
    assert_eq!(c.evaluate_for_take(GameTake::Cent(Take::CENT)), 0);
}

#[test]
fn test_evaluate_for_take_club() {
    let c = Card::new(CardColor::SPADES, CardFamily::JACK);
    assert_eq!(c.evaluate_for_take(GameTake::Club(Take::CLUBS)), 2);

    let c = Card::new(CardColor::CLUBS, CardFamily::JACK);
    assert_eq!(c.evaluate_for_take(GameTake::Club(Take::CLUBS)), 20);

    let c = Card::new(CardColor::HEARTS, CardFamily::JACK);
    assert_eq!(c.evaluate_for_take(GameTake::Club(Take::CLUBS)), 2);

    let c = Card::new(CardColor::DIAMONDS, CardFamily::JACK);
    assert_eq!(c.evaluate_for_take(GameTake::Club(Take::CLUBS)), 2);

    let c = Card::new(CardColor::SPADES, CardFamily::NINE);
    assert_eq!(c.evaluate_for_take(GameTake::Club(Take::CLUBS)), 0);

    let c = Card::new(CardColor::CLUBS, CardFamily::NINE);
    assert_eq!(c.evaluate_for_take(GameTake::Club(Take::CLUBS)), 14);

    let c = Card::new(CardColor::HEARTS, CardFamily::NINE);
    assert_eq!(c.evaluate_for_take(GameTake::Club(Take::CLUBS)), 0);

    let c = Card::new(CardColor::DIAMONDS, CardFamily::NINE);
    assert_eq!(c.evaluate_for_take(GameTake::Club(Take::CLUBS)), 0);
}

#[test]
fn test_evaluate_for_take_spades() {
    let c = Card::new(CardColor::SPADES, CardFamily::JACK);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 20);

    let c = Card::new(CardColor::CLUBS, CardFamily::JACK);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 2);

    let c = Card::new(CardColor::HEARTS, CardFamily::JACK);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 2);

    let c = Card::new(CardColor::DIAMONDS, CardFamily::JACK);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 2);

    let c = Card::new(CardColor::SPADES, CardFamily::NINE);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 14);

    let c = Card::new(CardColor::CLUBS, CardFamily::NINE);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 0);

    let c = Card::new(CardColor::HEARTS, CardFamily::NINE);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 0);

    let c = Card::new(CardColor::DIAMONDS, CardFamily::NINE);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 0);

    let c = Card::new(CardColor::SPADES, CardFamily::ACE);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 11);

    let c = Card::new(CardColor::SPADES, CardFamily::TEN);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 10);

    let c = Card::new(CardColor::SPADES, CardFamily::KING);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 4);

    let c = Card::new(CardColor::SPADES, CardFamily::QUEEN);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 3);

    let c = Card::new(CardColor::SPADES, CardFamily::HEIGHT);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 0);

    let c = Card::new(CardColor::SPADES, CardFamily::SEVEN);
    assert_eq!(c.evaluate_for_take(GameTake::Spade(Take::SPADE)), 0);
}

#[test]
fn test_evaluate_for_take_tout() {
    let c = Card::new(CardColor::SPADES, CardFamily::JACK);
    assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 20);

    let c = Card::new(CardColor::SPADES, CardFamily::NINE);
    assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 14);

    let c = Card::new(CardColor::SPADES, CardFamily::ACE);
    assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 11);

    let c = Card::new(CardColor::SPADES, CardFamily::TEN);
    assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 10);

    let c = Card::new(CardColor::SPADES, CardFamily::KING);
    assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 4);

    let c = Card::new(CardColor::SPADES, CardFamily::QUEEN);
    assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 3);

    let c = Card::new(CardColor::SPADES, CardFamily::HEIGHT);
    assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 0);

    let c = Card::new(CardColor::SPADES, CardFamily::SEVEN);
    assert_eq!(c.evaluate_for_take(GameTake::Tout(Take::TOUT)), 0);
}

#[test]
fn test_new_card() {
    let c = Card::new(CardColor::SPADES, CardFamily::JACK);

    assert_eq!(CardColor::SPADES, c.color);
    assert_eq!(CardFamily::JACK, c.family);
}

#[test]
fn test_is_ace() {
    let mut c = Card::new(CardColor::SPADES, CardFamily::JACK);
    assert_eq!(c.is_ace(), false);

    c = Card::new(CardColor::SPADES, CardFamily::TEN);
    assert_eq!(c.is_ace(), false);

    c = Card::new(CardColor::SPADES, CardFamily::ACE);
    assert_eq!(c.is_ace(), true);
}

#[test]
fn test_is_jack() {
    let mut c = Card::new(CardColor::SPADES, CardFamily::JACK);
    assert_eq!(c.is_jack(), true);

    c = Card::new(CardColor::SPADES, CardFamily::TEN);
    assert_eq!(c.is_jack(), false);

    c = Card::new(CardColor::SPADES, CardFamily::ACE);
    assert_eq!(c.is_jack(), false);
}

#[test]
fn test_is_nine() {
    let mut c = Card::new(CardColor::SPADES, CardFamily::NINE);
    assert_eq!(c.is_nine(), true);

    c = Card::new(CardColor::SPADES, CardFamily::TEN);
    assert_eq!(c.is_nine(), false);

    c = Card::new(CardColor::SPADES, CardFamily::ACE);
    assert_eq!(c.is_nine(), false);
}

#[test]
fn test_is_ten() {
    let mut c = Card::new(CardColor::SPADES, CardFamily::TEN);
    assert_eq!(c.is_ten(), true);

    c = Card::new(CardColor::SPADES, CardFamily::JACK);
    assert_eq!(c.is_ten(), false);

    c = Card::new(CardColor::SPADES, CardFamily::ACE);
    assert_eq!(c.is_ten(), false);
}
