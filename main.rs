use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards:Vec<String>,
}

//inherent implementation
impl Deck {
    fn new() -> Self {
    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];


    let mut cards = vec![];
    
    // Double nested loop
    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card)

        }
    }
    Deck { cards }
    }

    fn shuffle(&self) {

    }
}

fn main() {
    let deck = Deck::new();

    // format {:#?} helps print out to look right
    println!("Heres your deck: {:#?}", deck);
}
