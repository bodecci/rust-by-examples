
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        // If self is a Media::Book, we can use title and author here
        if let Media::Book { title, author } = self {
            format!("Book: {} {}", title, author)
            // If self is a Media::Movie, we can use title and director here
        } else if let Media::Movie{ title, director } = self {
            format!("Movie: {} {}", title, director)
            // If self is a Media::Audiobok, we can use title and director here
        } else if let Media::Audiobook {title} = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Media Description")   
        }
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

    println!("{:#?}", audiobook.description());
    println!("{:#?}", good_movie.description());
    println!("{:#?}", bad_book.description());


}
