use crate::cards::Card;
use crate::cards::{CardColor, CardFamily};
use crate::gametake::Take;
use crate::hand_test;

#[test]
fn test_evaluate_clubs() {
    assert_eq!(Take::CLUBS.evaluate(vec![]), (false, 0));
    assert_eq!(
        Take::CLUBS.evaluate(hand_test::thirty_four_color(CardColor::CLUBS)),
        (true, 0)
    );
    assert_eq!(
        Take::CLUBS.evaluate(hand_test::bad_hand_color(CardColor::CLUBS)),
        (false, 0)
    );
}
#[test]
fn test_evaluate_diamond() {
    assert_eq!(Take::DIAMOND.evaluate(vec![]), (false, 0));
    assert_eq!(
        Take::DIAMOND.evaluate(hand_test::thirty_four_color(CardColor::DIAMONDS)),
        (true, 0)
    );
    assert_eq!(
        Take::DIAMOND.evaluate(hand_test::bad_hand_color(CardColor::DIAMONDS)),
        (false, 0)
    );
}
#[test]
fn test_evaluate_heart() {
    assert_eq!(Take::HEART.evaluate(vec![]), (false, 0));
    assert_eq!(
        Take::HEART.evaluate(hand_test::thirty_four_color(CardColor::HEARTS)),
        (true, 0)
    );
    assert_eq!(
        Take::HEART.evaluate(hand_test::bad_hand_color(CardColor::HEARTS)),
        (false, 0)
    );
}
#[test]
fn test_evaluate_spade() {
    assert_eq!(Take::SPADE.evaluate(vec![]), (false, 0));
    assert_eq!(
        Take::SPADE.evaluate(hand_test::thirty_four_color(CardColor::SPADES)),
        (true, 0)
    );
    assert_eq!(
        Take::SPADE.evaluate(hand_test::bad_hand_color(CardColor::SPADES)),
        (false, 0)
    );
}

#[test]
fn test_evaluate_cent() {
    assert_eq!(Take::CENT.evaluate(vec![]), (false, 0));
    assert_eq!(
        Take::CENT.evaluate(hand_test::two_of_family(CardFamily::ACE)),
        (true, 0)
    );
    assert_eq!(
        Take::CENT.evaluate(hand_test::three_of_family(CardFamily::ACE)),
        (true, 0)
    );
    assert_eq!(
        Take::CENT.evaluate(hand_test::four_of_family(CardFamily::ACE)),
        (true, 0)
    )
}

#[test]
fn test_evaluate_tout() {
    assert_eq!(Take::TOUT.evaluate(vec![]), (false, 0));
    assert_eq!(
        Take::TOUT.evaluate(hand_test::two_of_family(CardFamily::JACK)),
        (true, 0)
    );
    assert_eq!(
        Take::TOUT.evaluate(hand_test::three_of_family(CardFamily::JACK)),
        (true, 0)
    );
    assert_eq!(
        Take::TOUT.evaluate(hand_test::four_of_family(CardFamily::JACK)),
        (true, 0)
    );
}
