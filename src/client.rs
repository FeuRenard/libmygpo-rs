use reqwest::blocking::{Client, Response};
use reqwest::IntoUrl;
use serde::Serialize;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct AuthenticatedClient {
    pub(crate) username: String,
    pub(crate) password: String,
    client: Client,
}

#[derive(Debug, Clone)]
pub struct DeviceClient {
    pub(crate) device_id: String,
    pub(crate) authenticated_client: AuthenticatedClient,
}

impl AuthenticatedClient {
    pub fn new(username: &str, password: &str) -> AuthenticatedClient {
        AuthenticatedClient {
            username: username.to_owned(),
            password: password.to_owned(),
            client: Client::new(),
        }
    }

    pub(crate) fn get<U: IntoUrl>(&self, url: U) -> Result<Response, reqwest::Error> {
        let empty_slice: &[&String] = &[];
        self.get_with_query(url, empty_slice)
    }

    pub(crate) fn get_with_query<U: IntoUrl, T: Serialize + ?Sized>(
        &self,
        url: U,
        query_parameters: &[&T],
    ) -> Result<Response, reqwest::Error> {
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

    pub(crate) fn put<T: Serialize + ?Sized, U: IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<Response, reqwest::Error> {
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

    pub(crate) fn post<T: Serialize + ?Sized, U: IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<Response, reqwest::Error> {
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

impl DeviceClient {
    pub fn new(username: &str, password: &str, device_id: &str) -> DeviceClient {
        DeviceClient {
            device_id: device_id.to_owned(),
            authenticated_client: AuthenticatedClient::new(username, password),
        }
    }

    pub(crate) fn get<U: IntoUrl>(&self, url: U) -> Result<Response, reqwest::Error> {
        self.authenticated_client.get(url)
    }

    pub(crate) fn get_with_query<U: IntoUrl, T: Serialize + ?Sized>(
        &self,
        url: U,
        query_parameters: &[&T],
    ) -> Result<Response, reqwest::Error> {
        self.authenticated_client
            .get_with_query(url, query_parameters)
    }

    pub(crate) fn put<T: Serialize + ?Sized, U: IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<Response, reqwest::Error> {
        self.authenticated_client.put(url, json)
    }

    pub(crate) fn post<T: Serialize + ?Sized, U: IntoUrl>(
        &self,
        url: U,
        json: &T,
    ) -> Result<Response, reqwest::Error> {
        self.authenticated_client.post(url, json)
    }
}
