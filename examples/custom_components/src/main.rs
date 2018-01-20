#[macro_use]
extern crate yew;

mod counter;
mod button;
mod barrier;

use yew::prelude::*;
use yew::html::Scope;
use yew::services::console::ConsoleService;
use counter::{Counter, Color};
use barrier::Barrier;

struct Context {
    console: ConsoleService,
}

/// If you use `App` you should implement this for `AppContext<Context, Model, Msg>` struct.
impl counter::Printer for Context {
    fn print(&mut self, data: &str) {
        self.console.log(data);
    }
}

struct CountButton {
    color: Color,
}

struct Model {
    count_buttons: Vec<CountButton>,
}

enum Msg {
    Repaint,
    ChildClicked(u32),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: &mut Env<Context, Self>) -> Self {
        Model {
            count_buttons: vec![CountButton{
                color: Color::Red,
            }]
        }
    }

    fn update(&mut self, msg: Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Repaint => {
                self.count_buttons.first().unwrap().color = Color::Blue;
                true
            }
            Msg::ChildClicked(value) => {
                context.console.log(&format!("child clicked: {}", value));
                false
            }
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        let counter = |model: &CountButton| html! {
            <Counter: color=model.color, onclick=Msg::ChildClicked,/>
        };
        html! {
            <div>
                <Barrier: limit=10, onsignal=|_| Msg::Repaint, />
                { for &self.count_buttons.iter().map(counter) }
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService,
    };
    // We use `Scope` here for demonstration.
    // You can also use `App` here too.
    let app: Scope<Context, Model> = Scope::new(context);
    app.mount_to_body();
    yew::run_loop();
}
