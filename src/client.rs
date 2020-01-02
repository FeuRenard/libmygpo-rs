const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct AuthenticatedClient {
    pub(crate) username: String,
    pub(crate) password: String,
    client: reqwest::Client,
}

impl AuthenticatedClient {
    pub fn new(username: &str, password: &str) -> AuthenticatedClient {
        AuthenticatedClient {
            username: username.to_owned(),
            password: password.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    pub(crate) fn get<U: reqwest::IntoUrl>(
        &self,
        url: U,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let empty_slice: &[&String] = &[];
        self.get_with_query(url, empty_slice)
    }

    pub(crate) fn get_with_query<U: reqwest::IntoUrl, T: serde::Serialize + ?Sized>(
        &self,
        url: U,
        query_parameters: &[&T],
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .query(query_parameters)
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

    pub(crate) fn post<T: serde::Serialize + ?Sized, U: reqwest::IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .post(url)
            .basic_auth(&self.username, Some(&self.password))
            .header(
                reqwest::header::USER_AGENT,
                &format!("{}/{}", PACKAGE_NAME, PACKAGE_VERSION),
            )
            .json(json)
            .send()
    }
}
