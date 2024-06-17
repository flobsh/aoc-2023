use std::cmp::Ordering;

use super::{Card, Hand, HandType};

pub trait CompareCards<T: Card> {
    fn cmp_cards(card_1: &T, card_2: &T) -> Ordering;
}

pub trait CompareHands<T: Hand> {
    fn cmp_hands(hand_1: &T, hand_2: &T) -> Ordering;
}

pub trait ComputeHandType<T: Hand> {
    fn hand_type(hand: &T) -> Option<HandType>;
}
