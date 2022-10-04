use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

struct App;
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div>
                <style>
{"
.title {
    width: 100%;
    text-align: center;
}
"}
                </style>
            <HelloWorld/>
            </div>
        }
    }
}

struct HelloWorld;
impl Component for HelloWorld {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <h1 class={"title"}>{"Hello World"}</h1>
        }
    }
}