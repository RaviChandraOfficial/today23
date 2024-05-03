use reqwest::Client;
use serde::{Serialize, Deserialize};
use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use crate::store;
use crate::router::Route;
use yew_router::prelude::Link;
#[function_component(PutData)]
pub fn put_data() -> Html {
    let (store, _store_dispatch) = use_store::<store::Store>();
    let id = use_state(|| 0);
    let name = use_state(|| "".to_string());
    let location = use_state(|| "".to_string());
    let data = use_state(|| "".to_string());
    let put_result = use_state(|| None::<String>);

    let put_data = {
        let id = id.clone();
        let name = name.clone();
        let location = location.clone();
        let data = data.clone();
        let put_result = put_result.clone();
        let store = store.clone(); 
        Callback::from(move |_| {
            let note = Note {
                id: (*id).clone(),
                name: (*name).clone(),
                location: (*location).clone(),
                data: (*data).clone(),
            };
            let token = store.token.clone(); 
            let put_result = put_result.clone();
            spawn_local(async move {
                let client = Client::new();
                let res = client.put("http://localhost:3000/put/user") // Modify the endpoint for PUT request
                    .header("AUTHORIZATION", token)
                    .json(&note)
                    .send()
                    .await;
                match res {
                    Ok(_response) => {
                        put_result.set(Some("Data updated successfully!".to_string()));
                    }
                    Err(e) => {
                        put_result.set(Some(format!("Error updating data: {:?}", e)));
                    }
                }
            });
        })
    };

    html! {
        <>
        <table style="width: 100%; background: #f0f0f0; border-bottom: 1px solid black;">
        <tr>
            <td style="text-align: center; border-right: 1px solid black;">
                <Link<Route> to={Route::Getdata}>{ "GET" }</Link<Route>>
            </td>
            <td style="text-align: center; border-right: 1px solid black;">
                <Link<Route> to={Route::PostData}>{ "POST" }</Link<Route>>
            </td>
            <td style="text-align: center; border-right: 1px solid black;">
                <Link<Route> to={Route::PutData}>{ "PUT" }</Link<Route>>
            </td>
            <td style="text-align: center;">
                <Link<Route> to={Route::DeleteData}>{ "DELETE" }</Link<Route>>
            </td>
        </tr>
        </table>
            <h1>{"Update Data"}</h1>
            <div>
                <input type="number" placeholder="ID"
                    onchange={Callback::from(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        id.set(input.value_as_number() as i32);
                    })}
                />
            </div>
            <div>
                <input type="text" placeholder="Name"
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        name.set(input.value());
                    })}
                />
            </div>
            <div>
                <input type="text" placeholder="Location"
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        location.set(input.value());
                    })}
                />
            </div>
            <div>
                <input type="text" placeholder="Data"
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        data.set(input.value());
                    })}
                />
            </div>
            <div>
                <button onclick={put_data}>{"Update Data"}</button>
            </div>
            <div>
                { if let Some(result) = (*put_result).as_ref() {
                    html! { <p>{ result }</p> }
                } else {
                    html! {}
                }}
            </div>
        </>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub data: String,
}
