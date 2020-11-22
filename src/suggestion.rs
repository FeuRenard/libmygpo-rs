//! [Suggestions API](https://gpoddernet.readthedocs.io/en/latest/api/reference/suggestions.html)

use crate::client::AuthenticatedClient;
use crate::client::DeviceClient;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use url::Url;

/// A podcast suggestion as returned by [retrieve_suggested_podcasts](RetrieveSuggestedPodcasts::retrieve_suggested_podcasts)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Suggestion {
    /// website of podcast
    pub website: Url,
    /// service-internal feed link
    pub mygpo_link: Url,
    /// description of podcast
    pub description: String,
    /// number of subscribers on service
    pub subscribers: u16,
    /// title of podcast
    pub title: String,
    /// feed URL
    pub url: Url,
    /// number of subscribers on service one week before
    pub subscribers_last_week: u16,
    /// URL to logo of podcast
    pub logo_url: Option<Url>,
}

/// see [retrieve_suggested_podcasts](RetrieveSuggestedPodcasts::retrieve_suggested_podcasts)
pub trait RetrieveSuggestedPodcasts {
    /// Retrieve Suggested Podcasts
    ///
    /// Download a list of podcasts that the user has not yet subscribed to (by checking all server-side subscription lists) and that might be interesting to the user based on existing subscriptions (again on all server-side subscription lists).
    ///
    /// The server does not specify the “relevance” for the podcast suggestion, and the client application SHOULD filter out any podcasts that are already added to the client application but that the server does not know about yet (although this is just a suggestion for a good client-side UX).
    ///
    /// # Examples
    ///
    /// ```
    /// use mygpoclient::client::AuthenticatedClient;
    /// use mygpoclient::suggestion::RetrieveSuggestedPodcasts;
    ///
    /// # let username = std::env::var("GPODDER_NET_USERNAME").unwrap();
    /// # let password = std::env::var("GPODDER_NET_PASSWORD").unwrap();
    /// #
    /// let client = AuthenticatedClient::new(&username, &password);
    ///
    /// let max_results = 3;
    /// let suggestions = client.retrieve_suggested_podcasts(max_results)?;
    ///
    /// assert!(suggestions.len() <= max_results as usize);
    ///
    /// # Ok::<(), mygpoclient::error::Error>(())
    /// ```
    ///
    /// # See also
    ///
    /// - [Suggestions API: Retrieve Suggested Podcasts](https://gpoddernet.readthedocs.io/en/latest/api/reference/suggestions.html#retrieve-suggested-podcasts)
    fn retrieve_suggested_podcasts(&self, max_results: u8) -> Result<Vec<Suggestion>, Error>;
}

impl RetrieveSuggestedPodcasts for AuthenticatedClient {
    fn retrieve_suggested_podcasts(&self, max_results: u8) -> Result<Vec<Suggestion>, Error> {
        Ok(self
            .get(&format!(
                "https://gpodder.net/suggestions/{}.json",
                max_results
            ))?
            .json()?)
    }
}

impl RetrieveSuggestedPodcasts for DeviceClient {
    fn retrieve_suggested_podcasts(&self, max_results: u8) -> Result<Vec<Suggestion>, Error> {
        self.as_ref().retrieve_suggested_podcasts(max_results)
    }
}

impl PartialEq for Suggestion {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl Eq for Suggestion {}

impl Ord for Suggestion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.url.cmp(&other.url)
    }
}

impl PartialOrd for Suggestion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Suggestion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}

impl fmt::Display for Suggestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} <{}>", self.title, self.description, self.url)
    }
}

#[cfg(test)]
mod tests {
    use super::Suggestion;
    use std::cmp::Ordering;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use url::Url;

    #[test]
    fn equal_suggestion_means_equal_hash() {
        let suggestion1 = Suggestion {
            url: Url::parse("http://goinglinux.com/mp3podcast.xml").unwrap(),
            website: Url::parse("http://www.linuxgeekdom.com").unwrap(),
            mygpo_link: Url::parse("http://gpodder.net/podcast/64439").unwrap(),
            description: String::from("Linux Geekdom"),
            subscribers: 0,
            title: String::from("Linux Geekdom"),
            subscribers_last_week: 0,
            logo_url: None,
        };
        let suggestion2 = Suggestion {
            url: Url::parse("http://goinglinux.com/mp3podcast.xml").unwrap(),
            website: Url::parse("http://goinglinux.com").unwrap(),
            mygpo_link: Url::parse("http://gpodder.net/podcast/11171").unwrap(),
            description: String::from("Going Linux"),
            subscribers: 571,
            title: String::from("Going Linux"),
            subscribers_last_week: 571,
            logo_url: Some(Url::parse("http://goinglinux.com/images/GoingLinux80.png").unwrap()),
        };

        assert_eq!(suggestion1, suggestion2);
        assert_eq!(suggestion1.partial_cmp(&suggestion2), Some(Ordering::Equal));

        let mut hasher1 = DefaultHasher::new();
        suggestion1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        suggestion2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn display() {
        let suggestion = Suggestion {
            url: Url::parse("http://goinglinux.com/mp3podcast.xml").unwrap(),
            website: Url::parse("http://goinglinux.com").unwrap(),
            mygpo_link: Url::parse("http://gpodder.net/podcast/11171").unwrap(),
            description: String::from("Going Linux"),
            subscribers: 571,
            title: String::from("Going Linux"),
            subscribers_last_week: 571,
            logo_url: Some(Url::parse("http://goinglinux.com/images/GoingLinux80.png").unwrap()),
        };

        assert_eq!(
            "Going Linux: Going Linux <http://goinglinux.com/mp3podcast.xml>".to_owned(),
            format!("{}", suggestion)
        );
    }
}
