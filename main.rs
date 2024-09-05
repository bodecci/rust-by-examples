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

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut  deck = Deck::new();

    deck.shuffle();

    // format {:#?} helps print out to look right
    println!("Heres your deck: {:#?}", deck);
}
