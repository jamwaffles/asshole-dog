use chrono::{DateTime, Utc};
use serde_json::json;
use yew::{
    format::json::Json,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

struct Model {
    fetch_service: FetchService,
    link: ComponentLink<Model>,
    fetching: bool,
    ft: Option<FetchTask>,
}

enum Msg {
    SendTimestamp,
    TimestampSaved,
    TimestampSaveFailed,
}

// TODO: Dedupe with backend
#[derive(Debug, serde_derive::Deserialize)]
struct OkResponse {
    ok: bool,
}

// TODO: Dedupe with request body in backend
#[derive(Debug, serde_derive::Serialize)]
pub struct AddTimestampPayload {
    timestamp: DateTime<Utc>,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            fetch_service: FetchService::new(),
            fetching: false,
            ft: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendTimestamp => {
                self.fetching = true;

                let cb = self.link.send_back(
                    move |response: Response<Json<Result<OkResponse, failure::Error>>>| {
                        let (meta, Json(_data)) = response.into_parts();

                        if meta.status.is_success() {
                            Msg::TimestampSaved
                        } else {
                            Msg::TimestampSaveFailed
                        }
                    },
                );

                let body = AddTimestampPayload {
                    timestamp: Utc::now(),
                };

                let request = Request::post("http://localhost:3001/api/add-timestamp")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("Failed to build request.");

                let ft = self.fetch_service.fetch(request, cb);

                self.ft = Some(ft);

                true
            }
            Msg::TimestampSaved => {
                self.fetching = false;

                true
            }
            Msg::TimestampSaveFailed => {
                self.fetching = false;

                println!("Error!");

                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let msg = if self.fetching {
            html! (<span>{"Wait..."}</span>)
        } else {
            html!(<span>{"Ready"}</span>)
        };

        html! {
            <div>
                <button onclick=|_| Msg::SendTimestamp>{ "Asshole!" }</button>
                {msg}
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
