//! Official English Pepper&Carrot episodes, with original credit pages.

use chek_extension::prelude::*;
use scraper::{Html, Selector};

const BASE: &str = "https://www.peppercarrot.com";
const SERIES: &str = "peppercarrot";
const EPISODES: &str = "https://www.peppercarrot.com/en/webcomic/";

struct Source;

impl Guest for Source {
    fn manifest() -> Manifest {
        Manifest {
            id: "page.chek.peppercarrot".into(),
            name: "Pepper&Carrot".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            sites: vec!["www.peppercarrot.com".into()],
            languages: vec!["en".into()],
            capabilities: vec![],
        }
    }

    fn search(query: String, page: u32, ratings: Vec<ContentRating>) -> Result<SeriesPage, Error> {
        let matches = query.split_whitespace().all(|word| {
            "pepper&carrot pepper and carrot david revoy fantasy magic witch comedy"
                .contains(&word.to_lowercase())
        });
        let items = if page == 1 && matches && ratings.contains(&ContentRating::Safe) {
            vec![SeriesItem {
                id: SERIES.into(),
                title: "Pepper&Carrot".into(),
                cover: Some(cover()),
                content_rating: Some(ContentRating::Safe),
            }]
        } else {
            vec![]
        };
        Ok(SeriesPage {
            items,
            has_more: false,
        })
    }

    fn series(id: String) -> Result<Series, Error> {
        check_series(&id)?;
        Ok(Series {
            id,
            title: "Pepper&Carrot".into(),
            summary: Some("The adventures of Pepper, a young witch, and her cat Carrot. A fantasy comedy for all ages.\n\nPepper&Carrot by David Revoy, licensed under Creative Commons Attribution 4.0 (https://creativecommons.org/licenses/by/4.0/). Original artwork and credit pages are retained without edits. Episode-specific contributors and proofreading credits: https://www.peppercarrot.com/en/webcomics/peppercarrot.html".into()),
            cover: Some(cover()),
            contributors: vec![Contributor { name: "David Revoy".into(), role: "author".into() }],
            reading_direction: Some(ReadingDirection::Ltr),
            status: Some(Status::Ongoing),
            year: Some(2014),
            tags: vec!["Fantasy".into(), "Comedy".into()],
            content_rating: Some(ContentRating::Safe),
        })
    }

    fn chapters(series_id: String) -> Result<Vec<Chapter>, Error> {
        check_series(&series_id)?;
        let body = Get::new(format!("{BASE}/en/webcomics/peppercarrot.html"))
            .cache(3600)
            .text()?;
        let document = Html::parse_document(&body);
        let links = selector("figure.translated figcaption a")?;
        let mut chapters = Vec::new();
        for link in document.select(&links) {
            let Some(id) = link
                .value()
                .attr("href")
                .and_then(|url| url.strip_prefix(EPISODES))
            else {
                continue;
            };
            let number = episode_number(id)?;
            chapters.push(Chapter {
                id: id.into(),
                title: link.text().collect::<String>().trim().into(),
                number: Some(number.to_string()),
                version: None,
                published: None,
                language: Some("en".into()),
                release_group: Some("David Revoy".into()),
                official: true,
            });
        }
        if chapters.is_empty() {
            return Err(Error::Parse("No Pepper&Carrot episodes found".into()));
        }
        chapters.sort_by_key(|chapter| episode_number(&chapter.id).unwrap_or_default());
        chapters.dedup_by(|a, b| a.id == b.id);
        Ok(chapters)
    }

    fn pages(chapter_id: String) -> Result<Vec<Image>, Error> {
        episode_number(&chapter_id)?;
        let body = Get::new(format!("{EPISODES}{chapter_id}"))
            .cache(3600)
            .text()?;
        let document = Html::parse_document(&body);
        let images = selector(".webcomic-page img")?;
        let mut pages = Vec::new();
        for element in document.select(&images) {
            let url = element
                .value()
                .attr("src")
                .filter(|url| url.starts_with("https://www.peppercarrot.com/0_sources/"))
                .ok_or_else(|| Error::Parse("Unexpected comic image URL".into()))?;
            pages.push(image(url));
        }
        if pages.is_empty() {
            return Err(Error::Parse("No comic pages found".into()));
        }
        Ok(pages)
    }

    fn transform_image(
        _: Image,
        _: chek_extension::http::Response,
    ) -> Result<Option<ImageData>, Error> {
        Ok(None)
    }
}

fn check_series(id: &str) -> Result<(), Error> {
    if id == SERIES {
        Ok(())
    } else {
        Err(Error::NotFound)
    }
}

fn episode_number(id: &str) -> Result<u32, Error> {
    if !id
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.'))
    {
        return Err(Error::NotFound);
    }
    let (number, title) = id
        .strip_prefix("ep")
        .and_then(|id| id.strip_suffix(".html"))
        .and_then(|id| id.split_once('_'))
        .ok_or(Error::NotFound)?;
    if title.is_empty() {
        return Err(Error::NotFound);
    }
    number
        .parse::<u32>()
        .ok()
        .filter(|number| *number > 0)
        .ok_or(Error::NotFound)
}

fn selector(value: &str) -> Result<Selector, Error> {
    Selector::parse(value).map_err(|error| Error::Parse(error.to_string()))
}

fn cover() -> Image {
    image(format!("{BASE}/core/img/header-peppercarrot.jpg"))
}

export_source!(Source);
