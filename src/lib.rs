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
    
    use std::{fs, io};
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
    lazy_static! {
        pub static ref CLIENT: MusicApi = client_init();
    }

    fn client_init() ->MusicApi {
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
    use crate::api;
    use futures_util::StreamExt;
    use rodio::{Decoder, OutputStream, Sink};
    use std::path::Path;
    
    use std::{fs::File, io::BufReader};

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

        pub fn restart(&self, id: u64) {
            let file = File::open(format!("cache/{}", id)).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            self.sink.stop();
            self.sink.append(source);
            self.sink.set_volume(0.3);
        }

        pub fn append(&self, id: u64) {
            // let file = File::open(path).unwrap();
            // let source = Decoder::new(BufReader::new(file)).unwrap();
            // self.sink.append(source);

            let file = File::open(format!("cache/{}", id)).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            dbg!(&rodio::Source::total_duration(&source));
            self.sink.append(source);
            dbg!(self.sink.len());
        }

        pub fn play(&self) {
            self.sink.play();
        }

        pub fn pause(&self) {
            self.sink.pause();
        }

        pub fn stop(&self) {
            self.sink.stop();
        }

        pub fn set_volume(&self, volume: f32) {
            self.sink.set_volume(volume);
        }
    }

    fn check_cache(id: u64) -> bool {
        dbg!(Path::new(format!("cache/{}", id).as_str()));
        if Path::new(format!("cache/{}", id).as_str()).exists() == true {
            dbg!("存在");
            true
        } else {
            dbg!("缓存不存在，开始下载");
            false
        }
    }
    async fn download(id: u64) -> Result<(), Box<dyn std::error::Error>> {
        dbg!("开始下载");
        let api = &api::CLIENT;
        dbg!("获取到api");
        let url = api.songs_url(&[id], "12800").await.unwrap()[0]
            .url
            .to_owned();
        dbg!(&url);
        let response = reqwest::get(url).await?;
        let mut file = File::create(format!("cache/{}", id))?;
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            std::io::Write::write_all(&mut file, &chunk)?;
        }
        Ok(())
    }
}
