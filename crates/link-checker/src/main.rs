use reqwest::blocking::{get, Response};
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

fn extract_links(response: Response) -> Result<Vec<Url>, Error> {
    let base_url = response.url().to_owned();
    let document = response.text()?;
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();

    let mut valid_urls = Vec::new();
    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(url) => valid_urls.push(url),
                Err(err) => {
                    println!("On {base_url}: could not parse {href:?}: {err} (ignored)",);
                }
            }
        }
    }

    Ok(valid_urls)
}

fn check_links(url: Url) -> Result<Vec<Url>, Error> {
    println!("Checking {url}");

    let response = get(url.to_owned())?;

    if !response.status().is_success() {
        return Ok(vec![url.to_owned()]);
    }

    let links = extract_links(response)?;
    for link in &links {
        println!("{link}, {:?}", link.domain());
    }

    let mut failed_links = Vec::new();
    for link in links {
        if link.domain() != url.domain() {
            println!("Checking external link: {link}");
            let response = get(link.clone())?;
            if !response.status().is_success() {
                println!("Error on {url}: {link} failed: {}", response.status());
                failed_links.push(link);
            }
        } else {
            println!("Checking link in same domain: {link}");
            failed_links.extend(check_links(link)?)
        }
    }

    Ok(failed_links)
}

fn main() {
    let start_url = Url::parse("https://www.google.org").unwrap();
    // let response = get(start_url).unwrap();
    // match extract_links(response) {
    //     Ok(links) => {
    //         println!("Links: {links:#?}");
    //         println!("total: {}", links.len());
    //     }
    //     Err(err) => println!("Could not extract links: {err:#}"),
    // }
    match check_links(start_url) {
        Ok(links) => println!("Links: {links:#?}"),
        Err(err) => println!("Could not extract links: {err:#}"),
    }
}
