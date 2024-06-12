use crate::components::{list::TrackList, loading::Loading};
use dioxus::prelude::*;
use lib::*;

#[component]
pub fn AlbumDetail(album_id: u64) -> Element {
    let future = use_resource(move || async move {
        let api = &api::CLIENT;
        let x = api.album(album_id).await;
        x
    });
    rsx! {
        match &*future.read() {
            Some(Ok(response)) => {
                rsx!{
                    div{id:"album_info_container",
                        img{class:"song_cover",
                        src: "{response.pic_url}",
                        }
                        div{id:"album_info",
                            h1{"{response.name}"}
                            p{id:"data&count"," 发行时间 {getdate(response.publish_time as i64)} · {response.songs.len()} 首歌 · {response.songs.iter().map(|e| e.duration/1000/60).sum::<u64>()} 分钟"}
                            p{"{response.description}"}
                        }
                    }
                    TrackList{tracks: response.songs.to_owned()}
                }
            }
            Some(Err(e)) => rsx!{
                p {"Error: {e}"}
            },
            None => rsx!{
                Loading {}
            }
        }


    }
}
