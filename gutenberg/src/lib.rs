//! A curated collection of English classics from Project Gutenberg.

use chek_extension::prelude::*;
use chek_extension::{
    Asset, ChapterContent, Content, ContentFormat, Reading, Rights, export_chapter_content,
};
use roxmltree::{Document, Node};

const BASE: &str = "https://www.gutenberg.org";
const DC: &str = "http://purl.org/dc/terms/";
const PG: &str = "http://www.gutenberg.org/2009/pgterms/";
const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
const PUBLIC_DOMAIN: &str = "Public domain in the USA.";
const BOOKS: &[(&str, &str, &str)] = &[
    ("1342", "Pride and Prejudice", "Jane Austen"),
    ("11", "Alice's Adventures in Wonderland", "Lewis Carroll"),
    (
        "84",
        "Frankenstein; or, The Modern Prometheus",
        "Mary Wollstonecraft Shelley",
    ),
    (
        "1661",
        "The Adventures of Sherlock Holmes",
        "Arthur Conan Doyle",
    ),
    ("2701", "Moby Dick; or, The Whale", "Herman Melville"),
    ("98", "A Tale of Two Cities", "Charles Dickens"),
    ("345", "Dracula", "Bram Stoker"),
    ("74", "The Adventures of Tom Sawyer", "Mark Twain"),
    ("1232", "The Prince", "Niccolò Machiavelli"),
    ("2600", "War and Peace", "Leo Tolstoy"),
    ("1184", "The Count of Monte Cristo", "Alexandre Dumas"),
    ("174", "The Picture of Dorian Gray", "Oscar Wilde"),
];

struct Source;

impl Guest for Source {
    fn manifest() -> Manifest {
        Manifest {
            id: "page.chek.gutenberg".into(),
            name: "Project Gutenberg Classics".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            sites: vec!["www.gutenberg.org".into()],
            languages: vec!["en".into()],
            capabilities: vec![],
        }
    }

    fn search(query: String, page: u32, _: Vec<ContentRating>) -> Result<SeriesPage, Error> {
        let words: Vec<_> = query
            .to_lowercase()
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        let items = BOOKS
            .iter()
            .filter(|(_, title, author)| {
                let text = format!("{title} {author}").to_lowercase();
                page == 1 && words.iter().all(|word| text.contains(word))
            })
            .map(|(id, title, _)| SeriesItem {
                id: (*id).into(),
                title: (*title).into(),
                cover: Some(cover(id)),
                // Gutenberg does not supply the SDK's sexual-content ratings.
                content_rating: None,
            })
            .collect();
        Ok(SeriesPage {
            items,
            has_more: false,
        })
    }

    fn series(id: String) -> Result<Series, Error> {
        let book = book(&id)?;
        Ok(Series {
            id: id.clone(),
            title: book.title,
            summary: Some(format!(
                "{}\n\n{}\nProject Gutenberg edition. Copyright terms differ outside the United States.\nSource: {BASE}/ebooks/{id}\nLicense: {BASE}/policy/license\n\n{}",
                book.summary, PUBLIC_DOMAIN, book.credits
            )),
            cover: Some(cover(&id)),
            contributors: book.authors,
            reading_direction: Some(ReadingDirection::Ltr),
            status: Some(Status::Completed),
            year: None,
            tags: book.subjects,
            content_rating: None,
        })
    }

    fn chapters(id: String) -> Result<Vec<Chapter>, Error> {
        let book = book(&id)?;
        Ok(vec![Chapter {
            id,
            title: book.title,
            number: None,
            version: Some(book.revision),
            published: book.issued,
            language: Some("en".into()),
            release_group: Some("Project Gutenberg".into()),
            official: true,
        }])
    }

    fn pages(_: String) -> Result<Vec<Image>, Error> {
        Err(Error::Other(
            "This book is an EPUB; use the chapter content endpoint.".into(),
        ))
    }

    fn transform_image(
        _: Image,
        _: chek_extension::http::Response,
    ) -> Result<Option<ImageData>, Error> {
        Ok(None)
    }
}

impl ChapterContent for Source {
    fn formats() -> Vec<ContentFormat> {
        vec![ContentFormat::Epub]
    }

    fn read(id: String) -> Result<Reading, Error> {
        let book = book(&id)?;
        Ok(Reading {
            content: Content::File(Asset {
                id: format!("{id}:{}:{}", book.url, book.revision),
                format: ContentFormat::Epub,
                url: book.url,
                headers: vec![],
                size: Some(book.size),
                file_name: Some(format!("pg{id}.epub")),
            }),
            rights: Some(Rights {
                license: Some(PUBLIC_DOMAIN.into()),
                license_url: Some(format!("{BASE}/policy/license")),
                attribution: Some(book.credits),
                source_url: Some(format!("{BASE}/ebooks/{id}")),
            }),
        })
    }
}

struct Book {
    title: String,
    summary: String,
    authors: Vec<Contributor>,
    subjects: Vec<String>,
    credits: String,
    issued: Option<String>,
    url: String,
    revision: String,
    size: u64,
}

fn book(id: &str) -> Result<Book, Error> {
    if !BOOKS.iter().any(|(known, _, _)| *known == id) {
        return Err(Error::NotFound);
    }
    let xml = Get::new(format!("{BASE}/cache/epub/{id}/pg{id}.rdf"))
        .cache(86400)
        .text()?;
    let doc = Document::parse(&xml).map_err(|e| Error::Parse(e.to_string()))?;
    let ebook = doc
        .descendants()
        .find(|n| {
            n.has_tag_name((PG, "ebook"))
                && n.attribute((RDF, "about")) == Some(format!("ebooks/{id}").as_str())
        })
        .ok_or_else(|| Error::Parse("Book metadata is missing".into()))?;
    if child_text(ebook, DC, "rights").as_deref() != Some(PUBLIC_DOMAIN) {
        return Err(Error::Other(
            "This edition is not marked public domain in the USA.".into(),
        ));
    }
    if !ebook
        .children()
        .filter(|n| n.has_tag_name((DC, "language")))
        .any(|n| value(n).as_deref() == Some("en"))
    {
        return Err(Error::Parse(
            "The selected edition is no longer in English".into(),
        ));
    }
    let title = child_text(ebook, DC, "title")
        .ok_or_else(|| Error::Parse("Book title is missing".into()))?;
    let authors: Vec<_> = ebook
        .children()
        .filter(|n| n.has_tag_name((DC, "creator")))
        .filter_map(|n| {
            n.descendants()
                .find(|n| n.has_tag_name((PG, "name")))
                .and_then(|n| n.text())
        })
        .map(|name| Contributor {
            name: name.into(),
            role: "author".into(),
        })
        .collect();
    let credits = format!(
        "{} — {}. Project Gutenberg. {}",
        title,
        authors
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        child_text(ebook, PG, "marc508").unwrap_or_default()
    );
    let mut formats: Vec<_> = ebook
        .descendants()
        .filter(|n| n.has_tag_name((PG, "file")))
        .filter(|n| {
            n.children()
                .filter(|n| n.has_tag_name((DC, "format")))
                .any(|n| value(n).as_deref() == Some("application/epub+zip"))
        })
        .filter_map(|n| n.attribute((RDF, "about")).map(|url| (n, url)))
        .filter(|(_, url)| url.starts_with(&format!("{BASE}/ebooks/{id}.epub")))
        .collect();
    formats.sort_by_key(|(_, url)| {
        if url.ends_with(".epub3.images") {
            0
        } else if url.ends_with(".epub.images") {
            1
        } else {
            2
        }
    });
    let (file, url) = formats
        .first()
        .ok_or_else(|| Error::Parse("No supported EPUB found".into()))?;
    let size = child_text(*file, DC, "extent")
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|size| *size > 0 && *size <= 128 * 1024 * 1024)
        .ok_or_else(|| {
            Error::Other("EPUB size is missing or exceeds Chek's 128 MiB limit".into())
        })?;
    let revision = child_text(*file, DC, "modified")
        .ok_or_else(|| Error::Parse("EPUB revision is missing".into()))?;
    Ok(Book {
        title,
        authors,
        credits,
        size,
        revision,
        url: (*url).into(),
        summary: child_text(ebook, PG, "marc520").unwrap_or_default(),
        issued: child_text(ebook, DC, "issued"),
        subjects: ebook
            .children()
            .filter(|n| n.has_tag_name((DC, "subject")))
            .filter_map(value)
            .collect(),
    })
}

fn child_text(node: Node<'_, '_>, namespace: &str, name: &str) -> Option<String> {
    node.children()
        .find(|n| n.has_tag_name((namespace, name)))
        .and_then(|n| n.text())
        .map(|s| s.trim().to_owned())
}

fn value(node: Node<'_, '_>) -> Option<String> {
    node.descendants()
        .find(|n| n.has_tag_name((RDF, "value")))
        .and_then(|n| n.text())
        .map(|s| s.trim().to_owned())
}

fn cover(id: &str) -> Image {
    image(format!("{BASE}/cache/epub/{id}/pg{id}.cover.medium.jpg"))
}

export_source!(Source);
export_chapter_content!(Source);
