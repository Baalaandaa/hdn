use std::net::{IpAddr, SocketAddr, TcpListener};
use std::sync::{Arc, RwLock};

use clap::Parser;

use crate::logic::connection::Connection;
use crate::logic::controller::Controller;
use crate::logic::log::{log_entry, log_load, log_store};
use crate::models::entry::EntryMessage;
use crate::models::request::Request;
use crate::models::response::{LoadResponse, StoreResponse};

mod logic;
mod models;

/// Main logic, connects Connection and Controller logic
pub fn listen(ip: IpAddr, port: u16) {
    let addr = SocketAddr::from((ip, port));
    let listener = TcpListener::bind(addr).unwrap();

    let controller = Controller::default();
    let controller = Arc::new(RwLock::new(controller));
    for stream in listener.incoming() {
        let mut connection = Connection::new(stream.unwrap());
        connection.write_string(
            serde_json::to_string(&EntryMessage {
                student_name: "Kirill Balandin (a.k.a. baalaandaa)".parse().unwrap(),
            })
            .unwrap(),
        );
        let controller = Arc::clone(&controller);
        log_entry(connection.name(), controller.read().unwrap().len());
        std::thread::Builder::new()
            .name(connection.name())
            .spawn(move || {
                while let Some(request) = connection.read::<Request>() {
                    let response = match request {
                        Request::Load { key } => {
                            log_load(connection.name(), &key, controller.read().unwrap().len());
                            let response = match controller.read().unwrap().get(&key) {
                                None => LoadResponse::KeyNotFound {},
                                Some(value) => LoadResponse::Success {
                                    requested_key: key,
                                    requested_hash: value,
                                },
                            };
                            serde_json::to_string(&response).unwrap()
                        }
                        Request::Store { key, hash } => {
                            let key_clone = key.clone();
                            let hash_clone = hash.clone();
                            controller.write().unwrap().insert(key, hash);
                            log_store(
                                connection.name(),
                                &key_clone,
                                &hash_clone,
                                controller.read().unwrap().len(),
                            );
                            serde_json::to_string(&StoreResponse::Success {}).unwrap()
                        }
                    };
                    connection.write_string(response);
                }
            })
            .unwrap()
            .join()
            .unwrap();
    }
}

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = "HDN is hash delivery network(a.k.a. KV storage). Uses TCP for communication"
)]
struct Opts {
    /// Listen ip
    #[clap(short, long, default_value = "0.0.0.0")]
    ip: IpAddr,

    /// Port to listen
    #[clap(short, long, default_value = "1337")]
    port: u16,
}

fn main() {
    let opts = Opts::parse();
    listen(opts.ip, opts.port);
}

#[cfg(test)]
mod test {
    use std::net::{IpAddr, TcpStream};
    use std::str::FromStr;
    use std::thread;
    use std::thread::JoinHandle;

    use crate::models::entry::EntryMessage;
    use crate::{listen, Connection, Request, StoreResponse};

    #[test]
    pub fn multiple_clients() {
        let addr = "127.0.0.1:1337";
        let _ = thread::spawn(move || {
            listen(IpAddr::from_str("127.0.0.1").unwrap(), 1337);
        })
        .thread()
        .unpark();

        let users_threads: Vec<JoinHandle<()>> = (0..100)
            .into_iter()
            .map(|idx| {
                thread::Builder::new()
                    .name(idx.to_string())
                    .spawn(move || {
                        let stream = TcpStream::connect(addr).unwrap();
                        let mut connection = Connection::new(stream);

                        let entry_message = connection.read::<EntryMessage>().unwrap();
                        assert_eq!(
                            entry_message.student_name,
                            "Kirill Balandin (a.k.a. baalaandaa)"
                        );

                        connection.write_string(
                            serde_json::to_string(&Request::Store {
                                key: idx.to_string(),
                                hash: "kekw".parse().unwrap(),
                            })
                            .unwrap(),
                        );

                        let store_response = connection.read::<StoreResponse>().unwrap();
                        assert!(matches!(store_response, StoreResponse::Success {}))
                    })
                    .unwrap()
            })
            .collect();
        for thread in users_threads {
            thread.join().unwrap();
        }
        return;
    }
}
