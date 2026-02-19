use super::{Format, Market};
use serde::{Deserialize, Serialize};
use serde_with::BoolFromInt;
use serde_with::{serde_as, skip_serializing_none};

#[skip_serializing_none]
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Response format.
    pub format: Option<Format>,

    #[serde(rename = "idx")]
    pub index: usize,

    #[serde(rename = "n")]
    pub number: usize,

    #[serde(rename = "mkt")]
    pub market: Option<Market>,

    #[serde_as(as = "Option<BoolFromInt>")]
    pub uhd: Option<bool>,
}

impl Query {
    pub fn new() -> Self {
        Query::default()
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = Some(format);

        self
    }

    pub fn format_option(mut self, format: Option<Format>) -> Self {
        self.format = format;

        self
    }
}

pub trait QueryParams: Sized {
    fn query(&self) -> &Query;
    
    fn query_mut(&mut self) -> &mut Query;

    fn index(mut self, index: usize) -> Self {
        self.query_mut().index = index;

        self
    }

    fn number(mut self, number: usize) -> Self {
        self.query_mut().number = number;

        self
    }

    fn market(mut self, market: Market) -> Self {
        self.query_mut().market = Some(market);

        self
    }

    fn market_option(mut self, market: Option<Market>) -> Self {
        self.query_mut().market = market;

        self
    }

    fn uhd(mut self, uhd: bool) -> Self {
        self.query_mut().uhd = Some(uhd);

        self
    }

    fn uhd_option(mut self, uhd: Option<bool>) -> Self {
        self.query_mut().uhd = uhd;

        self
    }
}

impl QueryParams for Query {
    fn query(&self) -> &Query {
        self
    }
    
    fn query_mut(&mut self) -> &mut Query {
        self
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            format: Some(Format::JSON),
            index: 0,
            number: 8,
            market: Some(Market::EN_US),
            uhd: Some(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        assert_eq!(
            serde_urlencoded::to_string(Query::new()).unwrap(),
            "format=js&idx=0&n=8&mkt=en-US&uhd=1"
        );

        assert_eq!(
            serde_urlencoded::to_string(
                Query::new()
                    .format(Format::HomePage)
                    .index(1)
                    .number(3)
                    .market(Market::ZH_CN)
                    .uhd(false)
            )
            .unwrap(),
            "format=hp&idx=1&n=3&mkt=zh-CN&uhd=0"
        );

        assert_eq!(
            serde_urlencoded::to_string(Query {
                format: None,
                index: 1,
                number: 3,
                market: None,
                uhd: None,
            })
            .unwrap(),
            "idx=1&n=3"
        );
    }
}
