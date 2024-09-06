// use staements: for import code from another crate(package)
// you can import multiple things on a single line using curly braces
use rand::{thread_rng, seq::SliceRandom};


// derive is an attribute. tels the compiler to add additional code to the struct
// Debug is a trait. has functions included that aid in debugging
#[derive(Debug)]
struct Deck {
    cards:Vec<String>,
}

//inherent implementation
// we add some functionality to the Deck struct
impl Deck {
     // the first argument determines whether we are 
     //making a method or an associated function
     // leaving of the first arguement, means we have an associated function
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

    // the first argument determines whether we are 
    //making a method or an associated function
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        //
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    // associated functions called using the "::" syntax
    let mut  deck = Deck::new();

    //deck.shuffle();
    let cards = deck.deal(3);

    // format {:#?} helps print out to look right
    println!("Heres your hand: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);
}
