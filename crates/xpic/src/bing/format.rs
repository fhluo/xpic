use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Format {
    #[serde(rename = "js")]
    JSON,
    #[serde(rename = "xml")]
    XML,
    #[serde(rename = "rss")]
    RSS,
    #[serde(rename = "hp")]
    HTML,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        #[derive(Debug, Serialize, Deserialize)]
        struct Query {
            format: Format,
        }

        fn query(format: Format) -> Query {
            Query { format }
        }

        assert_eq!(
            serde_urlencoded::to_string(query(Format::JSON)).unwrap(),
            "format=js"
        );
        assert_eq!(
            serde_urlencoded::to_string(query(Format::XML)).unwrap(),
            "format=xml"
        );
        assert_eq!(
            serde_urlencoded::to_string(query(Format::RSS)).unwrap(),
            "format=rss"
        );
        assert_eq!(
            serde_urlencoded::to_string(query(Format::HTML)).unwrap(),
            "format=hp"
        );
    }
}
