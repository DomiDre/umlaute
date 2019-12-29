#![recursion_limit="256"]
use yew::prelude::*;

mod umlaut_replacer;
use umlaut_replacer::replace_umlaute;

struct Model {
    entered_text: String,
    edited_text: String,
    entered_ignore_word: String,
    ignore_words: Vec<String>
}

enum Msg {
    SetText(String),
    AddIgnoreWord,
    AddIgnoreText(String)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            entered_text: String::new(),
            edited_text: String::new(),
            entered_ignore_word: String::new(),
            ignore_words: Vec::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetText(s) => {
                self.entered_text = s;
                self.perform_substitution();
            },
            Msg::AddIgnoreText(s) => {
                self.entered_ignore_word = s;
            },
            Msg::AddIgnoreWord => {
                self.ignore_words.push(self.entered_ignore_word.clone());
                self.entered_ignore_word = String::new();
                self.perform_substitution();
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <section class="app-container">
                <header>
                    <h1>{"Umlaute Substituter"}</h1>
                </header>
                <main>
                    <div>
                        <textarea type="text", placeholder="Enter Text",
                        oninput=|i| Msg::SetText(i.value)
                        rows="10", cols="80">
                            {&self.entered_text}
                        </textarea>
                    </div>
                    <div>
                        <ul>
                            { for self.ignore_words.iter().map(|word| self.view_ignore_word(word)) }
                        </ul>
                        <input type="text", placeholder="Add Ignore Word"
                        oninput=|i| Msg::AddIgnoreText(i.value)
                        value=&self.entered_ignore_word/>
                        <button onclick=|_| Msg::AddIgnoreWord>{ "ADD" }</button>
                    </div>
                </main>
                <output>
                    <textarea type="text"
                    rows="10", cols="80">
                        { &self.edited_text }
                    </textarea>
                </output>
            </section>
        }
    }

}

impl Model {
    fn view_ignore_word(&self, word: &String) -> Html<Model> {
        html! {
            <li>
                {word}
            </li>
        }
    }

    fn perform_substitution(&mut self) {
        self.edited_text = replace_umlaute(
            &self.entered_text,
            &self.ignore_words
        );
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}