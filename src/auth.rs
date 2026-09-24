//! Session authentication for D&D Beyond's internal API.
use reqwest::{blocking::Client, header, redirect::Policy};
use serde::Deserialize;
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

const AUTH_URL: &str = "https://auth-service.dndbeyond.com/v1/cobalt-token";

#[derive(Clone)]
pub struct Session {
    cookie: Zeroizing<String>,
    token: Zeroizing<String>,
    refresh_at: Instant,
}

impl Session {
    pub fn cookie(&self) -> Zeroizing<String> {
        self.cookie.clone()
    }

    /// The official message broker carries the short-term token in the handshake URL.
    /// Keep this request private and never log the URL or websocket errors.
    pub(crate) fn stream_request(
        &self,
        campaign: &str,
    ) -> Result<tungstenite::http::Request<()>, String> {
        use base64::Engine;
        use tungstenite::client::IntoClientRequest;
        if campaign.parse::<u64>().ok().is_none_or(|n| n == 0)
            || !campaign.bytes().all(|b| b.is_ascii_digit())
        {
            return Err("Invalid campaign ID.".into());
        }
        let payload = self
            .token
            .split('.')
            .nth(1)
            .ok_or("Invalid session token.")?;
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(payload)
            .map_err(|_| "Invalid session claims.")?;
        let claims: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|_| "Invalid session claims.")?;
        let user = claims["http://schemas.xmlsoap.org/ws/2005/05/identity/claims/nameidentifier"]
            .as_str()
            .ok_or("Session has no account ID.")?;
        if user.parse::<u64>().ok().is_none_or(|n| n == 0) {
            return Err("Invalid session account ID.".into());
        }
        let mut url = reqwest::Url::parse("wss://game-log-api-live.dndbeyond.com/v1")
            .map_err(|_| "Invalid event endpoint.")?;
        url.query_pairs_mut()
            .append_pair("gameId", campaign)
            .append_pair("userId", user)
            .append_pair("stt", self.token.as_str());
        url.as_str()
            .into_client_request()
            .map_err(|_| "Could not build the event connection.".into())
    }

    pub fn needs_refresh(&self) -> bool {
        Instant::now() >= self.refresh_at
    }

    pub(crate) fn website_request(
        &self,
        request: reqwest::blocking::RequestBuilder,
    ) -> Result<reqwest::blocking::RequestBuilder, String> {
        Ok(self
            .authorize(request)
            .header(header::COOKIE, cookie_header(&self.cookie)?))
    }

    /// Builds an authorized request without exposing the token to callers.
    pub fn authorize(
        &self,
        request: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        request.bearer_auth(self.token.as_str())
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    token: String,
    #[serde(default = "default_ttl")]
    ttl: u64,
}

fn default_ttl() -> u64 {
    300
}

fn cookie_header(cookie: &str) -> Result<header::HeaderValue, String> {
    // Accept the cookie value only, never arbitrary headers or extra cookies.
    if cookie.is_empty()
        || !cookie
            .bytes()
            .all(|b| b.is_ascii_graphic() && !matches!(b, b';' | b',' | b'"' | b'\\'))
    {
        return Err("Paste only the CobaltSession cookie value.".into());
    }
    let mut value = header::HeaderValue::from_str(&format!("CobaltSession={cookie}"))
        .map_err(|_| "The session cookie is not valid.".to_owned())?;
    value.set_sensitive(true);
    Ok(value)
}

pub fn connect(cookie: Zeroizing<String>) -> Result<Session, String> {
    exchange(AUTH_URL, cookie)
}

fn exchange(url: &str, cookie: Zeroizing<String>) -> Result<Session, String> {
    let value = cookie_header(&cookie)?;
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .redirect(Policy::none())
        .build()
        .map_err(|_| "Could not start the secure connection.".to_owned())?;
    let response = client
        .post(url)
        .header(header::COOKIE, value)
        .header(header::CONTENT_TYPE, "application/json")
        .send()
        .map_err(|_| {
            "Could not reach D&D Beyond. Check your connection and try again.".to_owned()
        })?;
    match response.status().as_u16() {
        200..=299 => {},
        401 | 403 => return Err("D&D Beyond rejected the session or blocked the request. Sign in again and copy a fresh cookie.".into()),
        429 => return Err("D&D Beyond is limiting requests. Wait before trying again.".into()),
        _ => return Err("D&D Beyond could not complete authentication. Try again later.".into()),
    }
    let data: TokenResponse = response
        .json()
        .map_err(|_| "D&D Beyond returned an unexpected authentication response.".to_owned())?;
    let token = Zeroizing::new(data.token);
    if token.trim().is_empty()
        || data.ttl == 0
        || header::HeaderValue::from_str(&format!("Bearer {}", token.as_str())).is_err()
    {
        return Err("D&D Beyond returned an invalid token.".into());
    }
    // Refresh early, and cap unknown service lifetimes at five minutes.
    let refresh_in = Duration::from_secs((data.ttl.min(300) * 4 / 5).max(1));
    Ok(Session {
        cookie,
        token,
        refresh_at: Instant::now() + refresh_in,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    fn serve(status: &str, body: &str) -> (String, thread::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let reply = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut request = Vec::new();
            let mut buf = [0; 1024];
            while !request.windows(4).any(|s| s == b"\r\n\r\n") {
                let n = stream.read(&mut buf).unwrap();
                assert!(n > 0);
                request.extend_from_slice(&buf[..n]);
            }
            stream.write_all(reply.as_bytes()).unwrap();
            String::from_utf8(request).unwrap()
        });
        (url, handle)
    }

    #[test]
    fn exchanges_cookie_and_authorizes_requests() {
        let (url, server) = serve("200 OK", r#"{"token":"test-token","ttl":3600}"#);
        let session = exchange(&url, Zeroizing::new("test-cookie".into())).unwrap();
        let request = server.join().unwrap();
        assert!(request.starts_with("POST / HTTP/1.1"));
        assert!(request.contains("cookie: CobaltSession=test-cookie"));
        assert!(!session.needs_refresh());
        let request = session
            .authorize(Client::new().get("https://example.com"))
            .build()
            .unwrap();
        assert_eq!(
            request.headers()[header::AUTHORIZATION],
            "Bearer test-token"
        );
    }

    #[test]
    fn rejects_bad_responses_without_exposing_the_body() {
        for (status, body) in [
            ("401 Unauthorized", "secret"),
            ("429 Too Many Requests", "secret"),
            ("200 OK", "not json"),
            ("200 OK", r#"{"token":"","ttl":3600}"#),
            ("200 OK", r#"{"token":"secret","ttl":0}"#),
            ("302 Found", "secret"),
        ] {
            let (url, server) = serve(status, body);
            let error = exchange(&url, Zeroizing::new("test-cookie".into()))
                .err()
                .unwrap();
            assert!(!error.contains("secret"));
            server.join().unwrap();
        }
    }

    #[test]
    fn rejects_cookie_injection() {
        for cookie in [
            "",
            "value; other=secret",
            "value\r\nX-Header: secret",
            "two values",
        ] {
            assert!(cookie_header(cookie).is_err());
        }
    }
}
