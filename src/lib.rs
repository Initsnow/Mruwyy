use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use chrono::{Local, TimeZone};

pub fn getdate(timestamp_in_millis: i64) -> String {
    Local
        .timestamp_millis_opt(timestamp_in_millis)
        .single()
        .unwrap()
        .format("%Y-%m-%d")
        .to_string()
}

pub mod api {
    use cookie_store::CookieStore;
    use dioxus_logger::tracing::{error, instrument};
    use lazy_static::lazy_static;
    pub use ncm_api::*;

    use std::{
        fs, io,
        sync::{Arc, Mutex},
    };
    const BASE_URL_LIST: [&str; 12] = [
        "https://music.163.com/",
        "https://music.163.com/eapi/clientlog",
        "https://music.163.com/eapi/feedback",
        "https://music.163.com/api/clientlog",
        "https://music.163.com/api/feedback",
        "https://music.163.com/neapi/clientlog",
        "https://music.163.com/neapi/feedback",
        "https://music.163.com/weapi/clientlog",
        "https://music.163.com/weapi/feedback",
        "https://music.163.com/wapi/clientlog",
        "https://music.163.com/wapi/feedback",
        "https://music.163.com/openapi/clientlog",
    ];
    const COOKIE_FILE: &str = "cookies.json";
    const MAX_CONS: usize = 32;
    #[derive(Debug)]
    pub struct UserLike {
        pub song: Arc<Mutex<Vec<u64>>>,
    }
    impl UserLike {
        pub async fn init(&self, song: Vec<u64>) {
            *self.song.lock().unwrap() = song;
        }
        pub async fn add(&self, song: u64) {
            self.song.lock().unwrap().push(song);
        }
        pub async fn remove(&self, song: u64) {
            self.song.lock().unwrap().retain(|&e| e != song);
        }
        pub fn check(&self, song: u64) -> bool {
            self.song.lock().unwrap().contains(&song)
        }
    }
    lazy_static! {
        pub static ref CLIENT: MusicApi = client_init();
        pub static ref LIKE_SONGS_LIST: UserLike = UserLike {
            song: Arc::new(Mutex::new(Vec::new()))
        };
    }

    fn client_init() -> MusicApi {
        if let Some(cookiejar) = load_cookie_jar_from_file() {
            return MusicApi::from_cookie_jar(cookiejar, MAX_CONS);
        }
        MusicApi::new(MAX_CONS)
    }

    #[instrument]
    pub fn save_cookie_jar_to_file(cookie_jar: CookieJar) {
        match fs::File::create(COOKIE_FILE) {
            Err(err) => error!("{:?}", err),
            Ok(mut file) => {
                let mut cookie_store = CookieStore::default();
                for base_url in BASE_URL_LIST {
                    let uri = &base_url.parse().unwrap();
                    let url = &base_url.parse().unwrap();
                    for c in cookie_jar.get_for_uri(url) {
                        let cookie = cookie_store::Cookie::parse(
                            format!(
                                "{}={}; Path={}; Domain=music.163.com; Max-Age=31536000",
                                c.name(),
                                c.value(),
                                url.path()
                            ),
                            uri,
                        )
                        .unwrap();
                        cookie_store.insert(cookie, uri).unwrap();
                    }
                }
                cookie_store.save_json(&mut file).unwrap();
            }
        }
    }
    #[instrument]
    pub fn load_cookie_jar_from_file() -> Option<CookieJar> {
        match fs::File::open(COOKIE_FILE) {
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => (),
                other => error!("{:?}", other),
            },
            Ok(file) => match CookieStore::load_json(io::BufReader::new(file)) {
                Err(err) => error!("{:?}", err),
                Ok(cookie_store) => {
                    let cookie_jar = CookieJar::default();
                    for base_url in BASE_URL_LIST {
                        let url = base_url.parse().unwrap();
                        for c in cookie_store.matches(&url) {
                            let cookie = CookieBuilder::new(c.name(), c.value())
                                .domain("music.163.com")
                                .path(c.path().unwrap_or("/"))
                                .build()
                                .unwrap();
                            cookie_jar.set(cookie, &base_url.parse().unwrap()).unwrap();
                        }
                    }
                    return Some(cookie_jar);
                }
            },
        };
        None
    }
}

pub mod play {

    use rodio::{Decoder, OutputStream, Sink};
    use std::time::Duration;

    use std::{fs::File, io::BufReader};

    use crate::TIME;

    pub struct Player {
        sink: Sink,
        _stream: OutputStream,
    }

    impl Player {
        pub fn new() -> Self {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            Self { sink, _stream }
        }

        pub fn restart(&mut self, id: u64) {
            let file = File::open(format!("cache/{}", id)).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            TIME.write()
                .unwrap()
                .update_total(rodio::Source::total_duration(&source).unwrap());
            self.sink.stop();
            self.sink.append(source);
            self.sink.set_volume(0.3);
            self.play();
        }

        pub fn append(&self, id: u64) {
            // let file = File::open(path).unwrap();
            // let source = Decoder::new(BufReader::new(file)).unwrap();
            // self.sink.append(source);

            let file = File::open(format!("cache/{}", id)).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            self.sink.append(source);
        }

        pub fn play(&self) {
            self.sink.play();
            TIME.write().unwrap().resume_or_flash();
        }

        pub fn pause(&self) {
            self.sink.pause();
            TIME.write().unwrap().pause();
        }

        pub fn stop(&self) {
            self.sink.stop();
        }

        pub fn set_volume(&self, volume: f32) {
            self.sink.set_volume(volume);
        }

        pub fn empty(&self) -> bool {
            self.sink.empty()
        }

        pub fn seek(&self, time: u64) {
            self.sink.try_seek(Duration::from_millis(time)).unwrap();
        }
    }
}

pub struct Time {
    pub total_time: Option<Duration>,
    pub instant: Option<Instant>,
    //true + \ false -
    offset: (u64, bool),
    tmp: Option<u64>,
}
use lazy_static::lazy_static;
lazy_static! {
    pub static ref TIME: Arc<RwLock<Time>> = Arc::new(RwLock::new(Time::new()));
}

impl Time {
    fn new() -> Self {
        Time {
            total_time: None,
            instant: None,
            offset: (0, false),
            tmp: None,
        }
    }
    fn update_total(&mut self, total: Duration) {
        self.total_time = Some(total);
    }
    pub fn flash(&mut self) {
        self.instant = Some(Instant::now());
        self.offset = (0, false);
    }
    pub fn get_current_millis(&self) -> u64 {
        if let Some(v) = self.instant {
            let offset = self.offset;
            if offset.1 {
                v.elapsed().as_millis() as u64 + offset.0
            } else {
                v.elapsed().as_millis() as u64 - offset.0
            }
        } else {
            0
        }
    }
    pub fn get_total_millis(&self) -> u64 {
        if let Some(v) = self.total_time {
            v.as_millis() as u64
        } else {
            0
        }
    }
    pub fn set(&mut self, to: u64) {
        if let Some(v) = self.instant {
            let v = v.elapsed().as_millis() as u64;
            if to >= v {
                self.offset = (to - v, true);
            } else {
                self.offset = (v - to, false);
            }
        }
    }
    fn pause(&mut self) {
        self.tmp = Some(self.get_current_millis());
    }
    fn resume_or_flash(&mut self) {
        if let Some(v) = self.tmp {
            self.set(v);
            println!("resume:{}", v);
            self.tmp = None;
        } else {
            self.flash();
        }
    }
}
