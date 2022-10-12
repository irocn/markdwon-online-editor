
use yew::prelude::*;
use pulldown_cmark::html::push_html;
use pulldown_cmark::{Options, Parser};
use yew::services::StorageService;
use yew::utils::document;
use yew::virtual_dom::VNode;
use log::info;
use yew::web_sys::{Element, Node, HtmlTextAreaElement};
use yew::{html, Component, Html, InputEvent, InputData,};
use serde_derive::{Deserialize, Serialize};
use yew::format::Json;
use yew::services::storage::{Area};

const KEY: &str = "yew.markdown.self";

pub struct App {
    link: ComponentLink<Self>,
    text: String,
    state: State,
    storage: StorageService,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    Change(String),
    Update(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let text = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                "".to_string()
            }
        };
        let state = State {
            text,
        };

        App {
            link,
            text: "".to_string(),
            state,
            storage,
        }
    }

    fn view(&self,) -> Html {
        let link = "https://github.com/pvcresin/yew-markdown-preview";
        let parse_html = parse_text(&self.state.text);
        let div: Element = document().create_element("div").unwrap();
        div.set_inner_html(&parse_html);
        div.set_class_name("preview");
        let node: Node = div.into();
        let preview = VNode::VRef(node);
        html! {
            <>
            <header id="header">
            {"MarkDown Editor v0.0.1"}
            </header>
        <article id="article">
            <div class="l-column">
                <h2 class="section-title">{"Markdown"}</h2>
                <textarea
                  class="markdown"
                  oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                  value=self.state.text.clone()
                />
            </div>
            <div class="l-column">
                <h2 class="section-title">{"Preview"}</h2>
                {preview}
            </div>
        </article>
            </>
        }
    }


    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Change(val) => {
                let mut options = Options::empty();
                options.insert(Options::ENABLE_TABLES);
                options.insert(Options::ENABLE_FOOTNOTES);
                options.insert(Options::ENABLE_STRIKETHROUGH);
                options.insert(Options::ENABLE_TASKLISTS);

                let parser = Parser::new_ext(&val, options);
                let mut parsed_text = String::new();
                push_html(&mut parsed_text, parser);
                self.text = parsed_text;
                //true
            },
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.text = val;
            }
        }

        self.storage.store(KEY, Json(&self.state.text));
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        todo!()
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}


}

fn parse_text(value: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&value, options);
    let mut parsed_text = String::new();
    push_html(&mut parsed_text, parser);

    parsed_text
}

