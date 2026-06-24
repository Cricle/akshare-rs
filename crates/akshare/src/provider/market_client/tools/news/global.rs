impl TradingToolbox {

    fn format_news_items(items: &[NewsItem]) -> String {
        items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                format!(
                    "{}. {} | {} | {}\n   Summary: {}\n   URL: {}",
                    index + 1,
                    item.published_at,
                    item.source,
                    item.title,
                    item.summary,
                    item.url.clone().unwrap_or_default()
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn unique_news_sources(items: &[NewsItem]) -> Vec<String> {
        let mut sources = items
            .iter()
            .map(|item| item.source.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        sources.sort();
        sources.dedup();
        sources
    }
}
