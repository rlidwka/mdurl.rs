use js_sys::Date;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let textarea = document.get_element_by_id("source").unwrap();

    let mut last_exec = 0f64;
    let timeout_ = std::rc::Rc::new(std::cell::Cell::new(None));

    //let timeout = timeout_.clone();
    let do_render = Closure::<dyn FnMut()>::new(move || {
        let textarea = textarea.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
        textarea.style().set_property("height", "0").unwrap();
        textarea.style().set_property("height", &format!("{}px", textarea.scroll_height())).unwrap();

        let src = textarea.value();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let maxlen = document.get_element_by_id("maxlen").unwrap();
        let maxlen = maxlen.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
        let maxlen = maxlen.value().parse().unwrap_or_default();

        let decoded = document.get_element_by_id("decoded").expect("no #decoded");
        let decoded = decoded.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
        decoded.set_inner_html(&mdurl::format_url_for_humans(&src, maxlen));
        decoded.style().set_property("height", "0").unwrap();
        decoded.style().set_property("height", &format!("{}px", decoded.scroll_height())).unwrap();

        let encoded = document.get_element_by_id("encoded").expect("no #encoded");
        let encoded = encoded.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
        encoded.set_inner_html(&mdurl::format_url_for_computers(&src));
        encoded.style().set_property("height", "0").unwrap();
        encoded.style().set_property("height", &format!("{}px", encoded.scroll_height())).unwrap();

        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes["InspiredGitHub"];
        let syntax = ss.find_syntax_by_extension("rs").unwrap();

        let debug = document.get_element_by_id("debug").expect("no #debug");
        let html = highlighted_html_for_string(&format!("{:#?}", mdurl::parse_url(&src)), &ss, syntax, theme);
        debug.set_inner_html(&html.unwrap_or_else(|_| "syntect failed, report this".into()));
    });

    let timeout = timeout_.clone();
    timeout.set(window.set_timeout_with_callback(do_render.as_ref().unchecked_ref()).ok());

    let timeout = timeout_;
    const DEBOUNCE_TIMEOUT : i32 = 20; // ms
    let input_handler = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
        let now = Date::now();
        if (now - last_exec).abs() < DEBOUNCE_TIMEOUT as f64 {
            if timeout.get().is_none() {
                timeout.set(
                    window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        do_render.as_ref().unchecked_ref(),
                        DEBOUNCE_TIMEOUT
                    ).ok()
                );
            }
            return;
        }

        last_exec = now;
        timeout.set(window.set_timeout_with_callback(do_render.as_ref().unchecked_ref()).ok());
    });

    let textarea = document.get_element_by_id("source").unwrap();
    textarea.add_event_listener_with_callback("input", input_handler.as_ref().unchecked_ref()).unwrap();
    let maxlen = document.get_element_by_id("maxlen").unwrap();
    maxlen.add_event_listener_with_callback("input", input_handler.as_ref().unchecked_ref()).unwrap();
    input_handler.forget();
}
