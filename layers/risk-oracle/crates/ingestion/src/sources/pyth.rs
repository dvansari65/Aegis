use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::time::SystemTime;
use url::Url;

use crate::{
    error::SourceError,
    models::{EventEnvelope, EventPayload, EventSource},
    sources::Source,
};

pub struct PythHermesSource {
    client: Client,
    hermes_url: Url,
    feed_ids: Vec<String>,
}

impl PythHermesSource {
    pub fn new(hermes_url: Url, feed_ids: Vec<String>) -> Self {
        Self {
            client: Client::new(),
            hermes_url,
            feed_ids,
        }
    }

    fn latest_price_url(&self) -> Result<Url, SourceError> {
        // The Hermes endpoint is assembled once per poll so feed selection stays
        // fully configuration-driven.
        let mut url = self
            .hermes_url
            .join("/v2/updates/price/latest")
            .map_err(|err| SourceError::MalformedResponse(err.to_string()))?;

        {
            let mut query = url.query_pairs_mut();
            query.append_pair("parsed", "true");
            for feed_id in &self.feed_ids {
                query.append_pair("ids[]", feed_id);
            }
        }

        Ok(url)
    }
}

#[async_trait]
impl Source for PythHermesSource {
    fn name(&self) -> &'static str {
        "pyth_hermes"
    }

    async fn fetch(&self) -> Result<Vec<EventEnvelope>, SourceError> {
        let observed_at = SystemTime::now();
        let response = self
            .client
            .get(self.latest_price_url()?)
            .send()
            .await?
            .error_for_status()?;

        let updates: Vec<HermesPriceUpdate> = response.json().await?;
        let mut events = Vec::with_capacity(updates.len());

        for update in updates {
            // Convert Pyth's transport shape into the shared event model as early
            // as possible so scoring logic never depends on Hermes-specific JSON.
            events.push(EventEnvelope::new(
                EventSource::PythHermes,
                EventPayload::OraclePriceSnapshot {
                    feed_id: update.id,
                    price: update.price.price,
                    confidence: update.price.confidence,
                    exponent: update.price.exponent,
                    publish_time: update.price.publish_time,
                    ema_price: update.ema_price.price,
                    ema_confidence: update.ema_price.confidence,
                },
                observed_at,
            ));
        }

        Ok(events)
    }
}

#[derive(Debug, Deserialize)]
struct HermesPriceUpdate {
    id: String,
    price: HermesPrice,
    #[serde(rename = "ema_price")]
    ema_price: HermesPrice,
}

#[derive(Debug, Deserialize)]
struct HermesPrice {
    #[serde(deserialize_with = "deserialize_i64")]
    price: i64,
    #[serde(rename = "conf", deserialize_with = "deserialize_u64")]
    confidence: u64,
    #[serde(rename = "expo")]
    exponent: i32,
    publish_time: i64,
}

fn deserialize_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    value.parse::<i64>().map_err(serde::de::Error::custom)
}

fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    value.parse::<u64>().map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::HermesPriceUpdate;

    #[test]
    fn parses_latest_price_response() {
        let payload = r#"
        [
          {
            "id": "feed-1",
            "price": {
              "price": "1000000",
              "conf": "25",
              "expo": -6,
              "publish_time": 1711111111
            },
            "ema_price": {
              "price": "999995",
              "conf": "20",
              "expo": -6,
              "publish_time": 1711111111
            }
          }
        ]
        "#;

        let parsed: Vec<HermesPriceUpdate> =
            serde_json::from_str(payload).expect("valid hermes response");

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].price.price, 1_000_000);
        assert_eq!(parsed[0].price.confidence, 25);
        assert_eq!(parsed[0].ema_price.price, 999_995);
    }
}
