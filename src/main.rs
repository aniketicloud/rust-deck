#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamond"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }
}

fn main() {
    let deck = Deck::new();
    println!("Here is your deck: {:#?}", deck)
}
