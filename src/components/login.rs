use crate::components::account::AccountInfo;
use crate::components::loading::Loading;
use crate::Route;
use dioxus::prelude::*;
use lib::api::{self, save_cookie_jar_to_file};
use qrcode_generator;
use std::error::Error;
use std::sync::RwLock;
use std::time::Duration;

#[component]
pub fn LoginWithUsernameAndPwd() -> Element {
    rsx! {
        div { class: "login_container",
            form { class:"login",onsubmit: move |event| {
            spawn(async move {
                println!("Clicked! {event:?}");
                let api=&api::CLIENT;
                let data=event.data;
                println!("{:?}",data.values().get("pwd"));
                dbg!(data.values().get("phone").unwrap().as_value(),data.values().get("pwd").unwrap().as_value());
                let result=api.login(data.values().get("phone").unwrap().as_value(),data.values().get("pwd").unwrap().as_value()).await;
                dbg!(result);
                dbg!(api.cookie_jar());
            });
         },
                input { r#type:"text", name:"phone", placeholder:"Username | Email | Phone", class:"login-input" },
                input { r#type:"password", name:"pwd", placeholder:"密码", class:"login-input" },
                button { r#type:"submit",class: "login-button", "登录"},
            }
        }
    }
}
fn checkLogin(mut error: Signal<Option<String>>) {
    spawn(async move {
        let api = &api::CLIENT;
        match api.login_status().await {
            Ok(login_info) => {
                dbg!(&login_info);
                save_cookie_jar_to_file(api.cookie_jar().unwrap().to_owned());
                let mut status = use_context::<Signal<RwLock<crate::Status>>>();
                status.write().write().unwrap().login = Some(AccountInfo {
                    name: login_info.nickname,
                    uid: login_info.uid,
                    avatar_url: login_info.avatar_url,
                });
                navigator().replace(Route::AccountDetail {});
                *error.write() = None;
            }
            Err(e) => *error.write() = Some(e.to_string()),
        }
    });
}
#[component]
pub fn Login() -> Element {
    let error = use_signal(|| None);
    rsx!(LoginWithQrcode { error: error })
}

#[component]
pub fn LoginWithQrcode(error: Signal<Option<String>>) -> Element {
    if let Some(e) = error() {
        return rsx! {"Login failed:\n{e}"};
    }
    let unikey = use_signal(String::new);
    let qrmsg = use_signal(String::new);
    let future: Resource<Result<String, Box<dyn Error>>> = use_resource(move || async move {
        let mut unikey = unikey.to_owned();
        let api = &api::CLIENT;
        let info = api.login_qr_create().await;
        let (a, b) = info.unwrap();
        unikey.set(b);
        spawn(async move {
            let mut qrmsg = qrmsg.to_owned();
            let api = &api::CLIENT;
            while let Ok(ref Msg) = api.login_qr_check(unikey.read().to_string()).await {
                if Msg.code == 803 {
                    qrmsg.set(Msg.msg.clone());
                    checkLogin(error);
                    break;
                } else {
                    qrmsg.set(Msg.msg.clone());
                }
                dbg!(qrmsg);
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        });
        Ok(qrcode_generator::to_svg_to_string(
            a,
            qrcode_generator::QrCodeEcc::Low,
            140,
            Some("wyy"),
        )
        .unwrap())
    });

    rsx! {
        match &*future.read() {
            Some(Ok(response)) => {
                // dbg!(&response);
            rsx!{
                div { class: "login_container",
            div{class:"qrcode_container",dangerous_inner_html:"{response}"},
            p{
                "{qrmsg.read()}"
            }
            button{
                onclick: move |event| {
                    spawn(async move {
                        println!("Clicked! {event:?}");
                        let api=&api::CLIENT;
                        let a:String = unikey.read().to_string();
                        let b=api.login_qr_check(a).await;
                        dbg!(b);
                        dbg!(api.cookie_jar());
                    });
                 },
                 "Click"
            }
        }

            }},
            Some(Err(e)) => rsx!{
                p {"Error: {e}"}
            },
            None => rsx!{
                Loading {}
            }
        }
    }
}
