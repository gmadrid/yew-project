use wasm_bindgen::JsCast;

pub fn download(text: &str, filename: &str) {
    // Figure out a better error handling solution here.
    let window = web_sys::window().expect("No window");
    let document = window.document().expect("no document");

    let mut data = "data:text/plain;charset=utf-8,".to_owned();
    data.push_str(text);

    let el = document.create_element("a").expect("no element created");
    el.set_attribute("href", &data).expect("couldn't set href");
    el.set_attribute("download", filename)
        .expect("couldn't set download");

    let body = document.body().expect("What? no Body?");
    body.append_child(&el).expect("couldn't append child");

    let html_el = el.dyn_into::<web_sys::HtmlElement>().expect("cast failed");
    html_el.click();
    body.remove_child(&html_el).expect("couldn't remove child");
}
