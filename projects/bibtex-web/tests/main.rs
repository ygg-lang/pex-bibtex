use bibtex::Bibliography;
use std::str::FromStr;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_tex_book() {
    let tex_book1 = "@book{texbook}";
    let bib = Bibliography::from_str(tex_book1).unwrap();
    assert_eq!(bib, Bibliography::new("texbook"));
    //
    let tex_book2 = "@book{texbook,}";
    let bib = Bibliography::from_str(tex_book2).unwrap();
    assert_eq!(bib, Bibliography::new("texbook"));

    let texbook = r#"
        @book{texbook,
        author = {Donald E. Knuth},
        year = {1986},
        title = {The {\TeX} Book},
        publisher = {Addison-Wesley Professional}
    }
    "#;
    let bib = Bibliography::from_str(texbook).unwrap();
    println!("{}", bib);
}

#[test]
fn test_tex_book2() {
    let bib = r#"
        @book{texbook,
        author = {Donald E. Knuth},
        year = {1986},
        title = {The {\TeX} Book},
        publisher = {Addison-Wesley Professional}
    }
    "#;

    let tex_book = Bibliography::from_str(bib).unwrap();
    println!("{:?}", tex_book.authors());
    println!("{}", serde_json::to_string_pretty(&tex_book).unwrap());
}
