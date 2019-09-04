use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

struct Model {
    fetch_service: FetchService,
    fetching: bool,
}

enum Msg {
    SendTimestamp,
    TimestampSaved,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            fetch_service: FetchService::new(),
            fetching: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendTimestamp => {
                self.fetching = true;

                let req = Request::post("https://localhost:3001/api/add-timestamp")
                    .header("Content-Type", "application/json")
                    .body(Json(&json!({"foo": "bar"})))
                    .expect("Failed to build request.");

                true
            }
            Msg::TimestampSaved => {
                self.fetching = false;

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
