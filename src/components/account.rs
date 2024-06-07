use std::error::Error;
use std::sync::{RwLock};

use crate::components::list::{UserFavoriteList, UserList};
use crate::components::loading::Loading;
use dioxus::prelude::*;
use lib::api;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct AccountInfo {
    pub name: String,
    pub avatar_url: String,
    pub uid: u64,
}

#[component]
pub fn AccountDetail() -> Element {
    let status = use_context::<Signal<RwLock<crate::Status>>>();
    if let Some(ref account_info) = status.to_owned().read().read().unwrap().login {
        let AccountInfo {
            name, avatar_url, ..
        } = account_info;
        use crate::components::list::Tracks;
        let future: Resource<Result<(Tracks, Tracks), Box<dyn Error>>> = use_resource(|| async {
            let status = use_context::<Signal<RwLock<crate::Status>>>();
            let api = &api::CLIENT;
            let mut name = String::new();
            let mut uid: u64 = 0;
            if let Some(ref account_info) = status.to_owned().read().read().unwrap().login {
                let AccountInfo {
                    name: name_,
                    uid: uid_,
                    ..
                } = account_info;
                name = name_.to_owned();
                uid = *uid_;
            }
            let lists = api.user_song_list(uid, 0, 100).await?;
            let tracks: Tracks = lists.into_iter().collect();
            let (userlists, userfavoritelists): (Tracks, Tracks) =
                tracks.vec.into_iter().partition(|e| e.author == name);
            Ok((userlists, userfavoritelists))
        });
        rsx! {
                    match &*future.read() {
                        Some(Ok(response)) => rsx!{
                            div{id:"user_info_container",
                                img{class:"avatar",
                                    src: "{avatar_url}",
                                }
                                div{id:"user_info",
                                    h1 {"{name}"}
                                }
                                div{id:"background",background:"url({avatar_url}) center"}
                            }
                            div{id:"user_list_container",
                                UserList{lists:response.0.to_owned()}
                                UserFavoriteList{lists:response.1.to_owned()}
                            }
                        },
                        Some(Err(e)) => rsx!{
                            p {"Error: {e}"}
                        },
                        None => rsx!{
                            Loading {}
                        }
                }
        }
    } else {
        rsx! {"账户未登录"}
    }
}
