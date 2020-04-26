//!
//! Types for representing cards, decks, and hands of German-style playing cards.
//!

use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Suit {
    Acorns,
    Leaves,
    Hearts,
    Bells,
}

// Internal, only for use in Deck creation.
static SUITS: [Suit; 4] = [Suit::Acorns, Suit::Leaves, Suit::Hearts, Suit::Bells];

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Face {
    Seven,
    Eight,
    Nine,
    Ten,
    LowerKnave,
    UpperKnave,
    King,
    Ace,
}

// Internal, only for use in Deck creation.
static FACES: [Face; 8] = [
    Face::Seven,
    Face::Eight,
    Face::Nine,
    Face::Ten,
    Face::LowerKnave,
    Face::UpperKnave,
    Face::King,
    Face::Ace,
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub face: Face,
}

/// A deck of cards that can be dealt from.
#[derive(Clone, Debug, Hash)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Create a random shuffle of the 32 possible cards.
    pub fn new_shuffled<R: Rng>(rng: &mut R) -> Deck {
        let mut cards = Vec::new();
        for suit in SUITS.iter() {
            for face in FACES.iter() {
                cards.push(Card {
                    suit: *suit,
                    face: *face,
                });
            }
        }

        cards.shuffle(rng);

        Deck { cards: cards }
    }

    /// The number of cards left in the deck.
    pub fn count(&self) -> usize {
        self.cards.len()
    }

    /// Take `num_cards` for a new `Hand`. The deck will have `num_cards` fewer
    /// cards after this call.
    ///
    /// # Panics
    ///
    /// Panics if there are fewer than `num_cards` left.
    pub fn deal(&mut self, num_cards: usize) -> Hand {
        assert!(num_cards <= self.count());
        Hand {
            cards: self.cards.split_off(self.count() - num_cards),
        }
    }
}

/// A player's hand of cards.
#[derive(Clone, Debug, Hash)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// All the cards currently in the hand.
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// The number of cards left.
    pub fn count(&self) -> usize {
        self.cards.len()
    }

    /// Play the card at `card_ndx`. This removes the card from the hand and
    /// returns it.
    ///
    /// # Panics
    ///
    /// Panics if `card_ndx` is out of the range `0..self.count()`
    pub fn play(&mut self, card_ndx: usize) -> Card {
        assert!(card_ndx < self.count());
        self.cards.remove(card_ndx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    use rand::rngs::mock::StepRng;

    #[test]
    fn deal() {
        let mut deck = Deck::new_shuffled(&mut StepRng::new(2, 5));
        assert_eq!(deck.count(), 32);

        let hand1 = deck.deal(12);
        assert_eq!(hand1.count(), 12);
        let hand2 = deck.deal(10);
        assert_eq!(hand2.count(), 10);
        let hand3 = deck.deal(10);
        assert_eq!(hand3.count(), 10);

        assert_eq!(deck.count(), 0);

        // Ensure all cards are unique (and by extension every possible card is
        // contained).
        let mut seen_cards = HashSet::<Card>::new();
        for hand in &[&hand1, &hand2, &hand3] {
            for card in hand.cards() {
                assert!(seen_cards.insert(*card));
            }
        }
    }

    #[test]
    fn play() {
        let mut rng = StepRng::new(3, 7);
        let mut deck = Deck::new_shuffled(&mut rng);
        let mut hand = deck.deal(10);

        // Ensure no card is taken twice.
        let mut seen_cards = HashSet::<Card>::new();
        for i in 0..10 {
            let card = hand.play(rng.gen_range(0, 10 - i));
            assert!(seen_cards.insert(card));
            assert_eq!(hand.count(), 9 - i);
        }
    }
}
