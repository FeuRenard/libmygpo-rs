const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Client {
    pub(crate) username: String,
    pub(crate) password: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(username: &str, password: &str) -> Client {
        Client {
            username: username.to_owned(),
            password: password.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    pub(crate) fn get<U: reqwest::IntoUrl>(
        &self,
        url: U,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .send()
    }

    pub(crate) fn put<T: serde::Serialize + ?Sized, U: reqwest::IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .put(url)
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .json(json)
            .send()
    }
}
