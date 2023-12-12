use crate::cards::Card;
use crate::cards::{CardColor, CardFamily};
use crate::gametake::{GameTake, Take};
use crate::hand_test;

#[test]
fn test_evaluate_clubs() {
    assert_eq!(
        Take::CLUBS.evaluate(vec![]),
        (false, GameTake::Skip(Take::SKIP), 0)
    );
    assert_eq!(
        Take::CLUBS.evaluate(hand_test::thirty_four_color(CardColor::CLUBS)),
        (true, GameTake::Club(Take::CLUBS), 0)
    );
    assert_eq!(
        Take::CLUBS.evaluate(hand_test::bad_hand_color(CardColor::CLUBS)),
        (false, GameTake::Skip(Take::SKIP), 0)
    );
}
#[test]
fn test_evaluate_diamond() {
    assert_eq!(
        Take::DIAMOND.evaluate(vec![]),
        (false, GameTake::Skip(Take::SKIP), 0)
    );
    assert_eq!(
        Take::DIAMOND.evaluate(hand_test::thirty_four_color(CardColor::DIAMONDS)),
        (true, GameTake::Diamond(Take::DIAMOND), 0)
    );
    assert_eq!(
        Take::DIAMOND.evaluate(hand_test::bad_hand_color(CardColor::DIAMONDS)),
        (false, GameTake::Skip(Take::SKIP), 0)
    );
}
#[test]
fn test_evaluate_heart() {
    assert_eq!(
        Take::HEART.evaluate(vec![]),
        (false, GameTake::Skip(Take::SKIP), 0)
    );
    assert_eq!(
        Take::HEART.evaluate(hand_test::thirty_four_color(CardColor::HEARTS)),
        (true, GameTake::Heart(Take::HEART), 0)
    );
    assert_eq!(
        Take::HEART.evaluate(hand_test::bad_hand_color(CardColor::HEARTS)),
        (false, GameTake::Skip(Take::SKIP), 0)
    );
}
#[test]
fn test_evaluate_spade() {
    assert_eq!(
        Take::SPADE.evaluate(vec![]),
        (false, GameTake::Skip(Take::SKIP), 0)
    );
    assert_eq!(
        Take::SPADE.evaluate(hand_test::thirty_four_color(CardColor::SPADES)),
        (true, GameTake::Spade(Take::SPADE), 0)
    );
    assert_eq!(
        Take::SPADE.evaluate(hand_test::bad_hand_color(CardColor::SPADES)),
        (false, GameTake::Skip(Take::SKIP), 0)
    );
}

#[test]
fn test_evaluate_cent() {
    assert_eq!(
        Take::CENT.evaluate(vec![]),
        (false, GameTake::Skip(Take::SKIP), 0)
    );
    assert_eq!(
        Take::CENT.evaluate(hand_test::two_of_family(CardFamily::ACE)),
        (true, GameTake::Cent(Take::CENT), 0)
    );
    assert_eq!(
        Take::CENT.evaluate(hand_test::three_of_family(CardFamily::ACE)),
        (true, GameTake::Cent(Take::CENT), 0)
    );
    assert_eq!(
        Take::CENT.evaluate(hand_test::four_of_family(CardFamily::ACE)),
        (true, GameTake::Cent(Take::CENT), 0)
    )
}

#[test]
fn test_evaluate_tout() {
    assert_eq!(
        Take::TOUT.evaluate(vec![]),
        (false, GameTake::Skip(Take::SKIP), 0)
    );
    assert_eq!(
        Take::TOUT.evaluate(hand_test::two_of_family(CardFamily::JACK)),
        (true, GameTake::Tout(Take::TOUT), 0)
    );
    assert_eq!(
        Take::TOUT.evaluate(hand_test::three_of_family(CardFamily::JACK)),
        (true, GameTake::Tout(Take::TOUT), 0)
    );
    assert_eq!(
        Take::TOUT.evaluate(hand_test::four_of_family(CardFamily::JACK)),
        (true, GameTake::Tout(Take::TOUT), 0)
    );
}
