//! Official campaign message-broker transport. Receives events; never publishes game events.
use crate::auth::Session;
use eframe::egui;
use serde_json::Value;
use std::{
    collections::VecDeque,
    net::{TcpStream, ToSocketAddrs},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
    },
    thread,
    time::{Duration, Instant},
};
use tungstenite::{Message, stream::MaybeTlsStream};
pub enum Event {
    Open,
    Changed(String),
    Closed,
}
pub struct Stream {
    pub rx: Receiver<Event>,
    cancel: Arc<AtomicBool>,
}
impl Drop for Stream {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}
impl Stream {
    pub fn start(session: Session, campaign: String, ctx: &egui::Context) -> Self {
        let (tx, rx) = mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let stop = cancel.clone();
        let ctx = ctx.clone();
        thread::spawn(move || {
            let _run = (|| -> Result<(), ()> {
                let request = session.stream_request(&campaign).map_err(|_| ())?;
                let addr = ("game-log-api-live.dndbeyond.com", 443)
                    .to_socket_addrs()
                    .map_err(|_| ())?
                    .next()
                    .ok_or(())?;
                let tcp =
                    TcpStream::connect_timeout(&addr, Duration::from_secs(10)).map_err(|_| ())?;
                tcp.set_read_timeout(Some(Duration::from_secs(10)))
                    .map_err(|_| ())?;
                tcp.set_write_timeout(Some(Duration::from_secs(10)))
                    .map_err(|_| ())?;
                let config = tungstenite::protocol::WebSocketConfig::default()
                    .max_message_size(Some(256 * 1024))
                    .max_frame_size(Some(256 * 1024));
                let (mut socket, _) =
                    tungstenite::client_tls_with_config(request, tcp, Some(config), None)
                        .map_err(|_| ())?;
                let tcp = match socket.get_mut() {
                    MaybeTlsStream::Plain(s) => s,
                    MaybeTlsStream::Rustls(s) => &mut s.sock,
                    _ => return Err(()),
                };
                tcp.set_read_timeout(Some(Duration::from_secs(1)))
                    .map_err(|_| ())?;
                if stop.load(Ordering::Relaxed) {
                    let _ = socket.close(None);
                    return Ok(());
                }
                tx.send(Event::Open).map_err(|_| ())?;
                ctx.request_repaint();
                let mut heartbeat = Instant::now();
                let mut received = Instant::now();
                let mut seen = VecDeque::new();
                while !stop.load(Ordering::Relaxed) && !session.needs_refresh() {
                    if heartbeat.elapsed() >= Duration::from_secs(300) {
                        socket
                            .send(Message::Text("{\"data\":\"ping\"}".into()))
                            .map_err(|_| ())?;
                        heartbeat = Instant::now();
                    }
                    if received.elapsed() > Duration::from_secs(345) {
                        break;
                    }
                    match socket.read() {
                        Ok(Message::Text(text)) => {
                            received = Instant::now();
                            if text == "pong" {
                                continue;
                            }
                            if let Some((event, id)) = change(&text, &campaign) {
                                if seen.contains(&event) {
                                    continue;
                                }
                                seen.push_back(event);
                                if seen.len() > 256 {
                                    seen.pop_front();
                                }
                                if tx.send(Event::Changed(id)).is_err() {
                                    break;
                                }
                                ctx.request_repaint();
                            }
                        }
                        Ok(Message::Ping(_)) => {
                            received = Instant::now();
                            socket.flush().map_err(|_| ())?;
                        }
                        Ok(Message::Pong(_)) => received = Instant::now(),
                        Ok(Message::Close(_)) => break,
                        Ok(_) => {}
                        Err(tungstenite::Error::Io(e))
                            if matches!(
                                e.kind(),
                                std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                            ) => {}
                        Err(_) => break,
                    }
                }
                let _ = socket.close(None);
                Ok(())
            })();
            let _ = tx.send(Event::Closed);
            ctx.request_repaint();
        });
        Self { rx, cancel }
    }
}
fn identifier(v: &Value) -> Option<String> {
    v.as_str()
        .map(str::to_owned)
        .or_else(|| v.as_u64().map(|n| n.to_string()))
}
pub fn change(text: &str, campaign: &str) -> Option<(String, String)> {
    let v: Value = serde_json::from_str(text).ok()?;
    if identifier(&v["gameId"]).as_deref() != Some(campaign) {
        return None;
    }
    let event = identifier(&v["id"])?;
    if v["eventType"] == "character-sheet/item-shared/fulfilled" {
        return Some((event, "*".into()));
    }
    if v["eventType"] != "character-sheet/character-update/fulfilled" {
        return None;
    }
    let id = identifier(&v["data"]["characterId"])?;
    id.parse::<u64>().ok().filter(|n| *n > 0)?;
    Some((event, id))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_matching_character_updates_trigger_reads() {
        assert_eq!(
            change(
                r#"{"id":"a","eventType":"character-sheet/character-update/fulfilled","gameId":"12","data":{"characterId":34}}"#,
                "12"
            ),
            Some(("a".into(), "34".into()))
        );
        assert!(change(r#"{"id":"a","eventType":"dice/roll/fulfilled","gameId":12,"data":{"characterId":34}}"#,"12").is_none());
        assert!(change(r#"{"id":"a","eventType":"character-sheet/character-update/fulfilled","gameId":13,"data":{"characterId":34}}"#,"12").is_none());
    }
}

#[cfg(test)]
mod live_monitor {
    use super::*;
    #[test]
    #[ignore = "Temporary read-only listener for a user-initiated sheet edit; waits up to 3 minutes"]
    fn listen_for_user_edit() {
        let dirs = directories::ProjectDirs::from("", "", "splat").unwrap();
        let config: Value = serde_json::from_slice(
            &std::fs::read(dirs.config_dir().join("settings.json")).unwrap(),
        )
        .unwrap();
        let campaign = config["campaigns"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["id"] == config["active_campaign_id"])
            .unwrap();
        let remote = campaign["dndbeyond_campaign_id"].as_str().unwrap();
        let session = crate::auth::connect(zeroize::Zeroizing::new(
            keyring::Entry::new("splat", "dndbeyond-session")
                .unwrap()
                .get_password()
                .unwrap(),
        ))
        .unwrap();
        let stream = Stream::start(session, remote.into(), &egui::Context::default());
        let started = Instant::now();
        let mut updates = 0;
        while started.elapsed() < Duration::from_secs(180) {
            match stream.rx.recv_timeout(Duration::from_secs(1)) {
                Ok(Event::Open) => {
                    println!("Listener connected; waiting for a user-initiated character update.")
                }
                Ok(Event::Changed(_)) => {
                    updates += 1;
                    println!("Received a matching character-update event from the live campaign.");
                }
                Ok(Event::Closed) => {
                    println!("Listener closed.");
                    break;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(_) => break,
            }
        }
        println!("Observed {updates} character-update events. No remote writes were made.");
    }
}
