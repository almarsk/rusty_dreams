use yew::prelude::*;

#[derive(Debug)]
pub enum Msg {
    SendMessage,
}

pub struct App {
    messages: Vec<String>,
    input_text: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        App {
            messages: Vec::new(),
            input_text: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SendMessage => {
                if !self.input_text.trim().is_empty() {
                    self.messages.push(self.input_text.clone());
                    self.input_text.clear();
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <ul>
                    { for self.messages.iter().map(|message| html! { <li>{ message }</li> }) }
                </ul>
                <div>
                    <input
                        type="text"
                    />
                    <button onclick={ctx.link().callback(|_| Msg::SendMessage)}>{"Send"}</button>
                </div>
            </div>
        }
    }
}
