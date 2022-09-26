use crate::input;
use anyhow::{anyhow, Result};
use cli_table::{Cell, ColorChoice, Style, Table};
use scraper::{Html, Selector};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct Ogp {
    pub items: Vec<(String, String)>,
}

impl Ogp {
    pub fn from(html: &str) -> Result<Self> {
        let dom = Html::parse_document(html);
        let selector = Selector::parse(r#"meta[property^="og"]"#).unwrap();

        let items = dom.select(&selector);
        let items = items
            .flat_map(|x| {
                let property = x.value().attr("property");
                let content = x.value().attr("content");

                if let (Some(property), Some(content)) = (property, content) {
                    vec![(String::from(property), String::from(content))]
                } else {
                    vec![]
                }
            })
            .collect::<Vec<_>>();

        Ok(Self { items })
    }

    pub fn render(self) -> Result<String> {
        let hashmap = self.items.into_iter().collect::<HashMap<_, _>>();
        serde_json::to_string_pretty(&hashmap).map_err(|e| anyhow!(e))
    }
}

impl Display for Ogp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let table = self
            .items
            .iter()
            .map(|(k, v)| vec![k, v])
            .table()
            .title(vec![
                "property".cell().bold(true),
                "content".cell().bold(true),
            ])
            .bold(true)
            .color_choice(if input::is_redirect() {
                ColorChoice::Never
            } else {
                ColorChoice::Auto
            });

        let display = table.display().map_err(|_| std::fmt::Error)?;

        write!(f, "{}", display)
    }
}
