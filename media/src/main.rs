
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast (u32),
    Placeholder
}

impl Media {
    fn description(&self) -> String {
        // // If self is a Media::Book, we can use title and author here
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        //     // If self is a Media::Movie, we can use title and director here
        // } else if let Media::Movie{ title, director } = self {
        //     format!("Movie: {} {}", title, director)
        //     // If self is a Media::Audiobok, we can use title and director here
        // } else if let Media::Audiobook {title} = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media Description")   
        // }
        match self{
            // use pattern matching with match. This allows you to check which variant 
            // of the enum you're dealing with and extract the associated data 
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media)
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let audiobook = Media::Audiobook { 
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good director")
    };
    let bad_book = Media::Book { 
        title: String::from("Bad Book"),
        author: String::from("Bad Author"), 
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{:#?}", audiobook.description());
    // println!("{:#?}", good_movie.description());
    // println!("{:#?}", bad_book.description());

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // running catalog.items.get(0), give an output with the built in Some like
    // Some(
    //    Audiobook { title: "An Audiobook", to take the some away, one needs to account for it with the pattern
    // matching
    match catalog.items.get(100) {
        //Some(value)
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        // None
        None => {
            println!("Nothing at that index")
        }
    }

}
