/// The book type provided by an external API.
#[derive(Debug)]
struct APIBook {
    title: String,
    description: Option<String>,
}

/// The book type you need in the rest of your program.
#[derive(Debug)]
struct Book {
    title: String,
    description: String,
}

fn main() {
    // The book objects you "received" from an API.
    let api_books: Vec<APIBook> = vec![
        APIBook {
            title: "Samson and Rik".to_string(),
            description: Some("Samson and Rik go on many adventures.".to_string()),
        },
        APIBook {
            title: "De Kameleon".to_string(),
            description: None,
        },
    ];

    println!("api_books: {api_books:#?}");

    // The book objects you would like to use throughout the rest of your program.
    let books: Vec<Book> = api_books
        .into_iter()
        .filter_map(|api_book| {
            // Deconstruct the APIBook into its parts.
            let APIBook { title, description } = api_book;

            // Return None if description is None, otherwise take the String out of the `Option<String>`.
            let description = match description {
                Some(description) => description,
                None => return None,
            };

            // Create Book from the parts.
            Some(Book { title, description })
        })
        .collect::<Vec<_>>();

    println!("books: {books:#?}");
}
