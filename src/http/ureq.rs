//! The client implementation for the ureq HTTP client, which is blocking.

use super::{headers, BaseClient, Form, Headers, Query};
use crate::client::{ClientError, ClientResult, Spotify};

use maybe_async::sync_impl;
use serde_json::Value;
use ureq::{Request, Response};

impl ClientError {
    pub fn from_response(r: ureq::Response) -> Self {
        ClientError::StatusCode(r.status(), r.status_text().to_string())
    }
}

impl Spotify {
    fn request<D>(
        &self,
        request: &mut Request,
        headers: Option<&Headers>,
        send_request: D,
    ) -> ClientResult<String>
    where
        D: Fn(&mut Request) -> Response,
    {
        // Setting the headers, which will be the token auth if unspecified.
        match headers {
            Some(headers) => {
                for (key, val) in headers.iter() {
                    request.set(&key, &val);
                }
            }
            None => {
                let (key, val) = headers::bearer_auth(self.get_token()?);
                request.set(&key, &val);
            }
        }

        log::info!("Making request {:?}", request);
        let response = send_request(request);

        if response.ok() {
            response.into_string().map_err(Into::into)
        } else {
            Err(ClientError::from_response(response))
        }
    }
}

#[sync_impl]
impl BaseClient for Spotify {
    #[inline]
    fn get(&self, url: &str, headers: Option<&Headers>, payload: &Query) -> ClientResult<String> {
        self.request(
            &mut ureq::get(&self.endpoint_url(url)),
            headers,
            |mut req| {
                for (key, val) in payload.iter() {
                    req = req.query(&key, &val)
                }
                req.call()
            },
        )
    }

    #[inline]
    fn post(&self, url: &str, headers: Option<&Headers>, payload: &Value) -> ClientResult<String> {
        self.request(&mut ureq::post(&self.endpoint_url(url)), headers, |req| {
            req.send_json(payload.clone())
        })
    }

    #[inline]
    fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form,
    ) -> ClientResult<String> {
        let payload = payload
            .iter()
            .map(|(key, val)| (key.as_str(), val.as_str()))
            .collect::<Vec<_>>();

        self.request(&mut ureq::post(&self.endpoint_url(url)), headers, |req| {
            req.send_form(&payload)
        })
    }

    #[inline]
    fn put(&self, url: &str, headers: Option<&Headers>, payload: &Value) -> ClientResult<String> {
        self.request(&mut ureq::put(&self.endpoint_url(url)), headers, |req| {
            req.send_json(payload.clone())
        })
    }

    #[inline]
    fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(&mut ureq::delete(&self.endpoint_url(url)), headers, |req| {
            req.send_json(payload.clone())
        })
    }
}
