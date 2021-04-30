#[macro_use]
extern crate diesel;

use std::str::FromStr;

use diesel::dsl::exists;
use diesel::prelude::*;
use diesel::select;

use sicniki_sumniki::{establish_connection, service::confirm_text};

diesel::table! {
    sources {
        id -> Integer,
        url -> Text,
        content_css_selector -> Text,
        title -> Text,
        enabled -> Bool,
    }
}

diesel::table! {
    visited_sources {
        id -> Integer,
        source_id -> Integer,
        url -> Text,
    }
}

#[derive(Queryable, Debug)]
struct Source {
    id: i32,
    url: String,
    content_css_selector: String,
    title: String,
    enabled: bool,
}

#[derive(Queryable, Debug)]
struct VisitedSource {
    id: i32,
    source_id: i32,
    url: String,
}

#[derive(Insertable)]
#[table_name = "visited_sources"]
struct NewVisitedSource {
    source_id: i32,
    url: String,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let connection = establish_connection();

    for s in sources::table
        .filter(sources::enabled.eq(true))
        .load::<Source>(&connection)
        .unwrap()
    {
        let r = ureq::get(&s.url).call().unwrap().into_string().unwrap();
        let c = rss::Channel::from_str(&r).unwrap();

        log::info!("processing source: {}", s.title);

        for i in c.items {
            log::info!("processing item: {}", i.title.unwrap_or_default());

            let l = i.link.as_ref().unwrap();

            let already_processed = select(exists(
                visited_sources::table.filter(visited_sources::url.eq(&l)),
            ))
            .get_result(&connection)
            .unwrap();

            if already_processed {
                log::info!("skipping, already processed.");
                continue;
            }

            let h = ureq::get(&l).call().unwrap().into_string().unwrap();
            let d = scraper::Html::parse_document(&h);
            let sel = scraper::Selector::parse(&s.content_css_selector).unwrap();

            for e in d.select(&sel) {
                let t = e.text().collect::<String>().to_owned();
                confirm_text(&t).unwrap();
            }

            let new_visited_source = NewVisitedSource {
                source_id: s.id,
                url: l.to_string(),
            };

            log::info!("saving item as processed.");
            diesel::insert_into(visited_sources::table)
                .values(&new_visited_source)
                .execute(&connection)
                .unwrap();
        }
    }
}
