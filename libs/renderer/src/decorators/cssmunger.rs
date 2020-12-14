use web_sys::CssStyleSheet;

pub struct CssMunger {
    css: CssStyleSheet,
}

impl CssMunger {
    pub fn new() -> Self {
        let css = CssMunger::find_css_stylesheet()
            .or_else(CssMunger::make_css_stylesheet)
            .expect("Coundn't acquire CSS style sheet.");

        CssMunger { css }
    }

    pub fn insert_rule(&self, rule: &str) {
        self.css.insert_rule(rule);
    }

    fn tester(&self) {
        self.css.insert_rule("body { background: purple }");
    }

    fn find_css_stylesheet() -> Option<CssStyleSheet> {
        use wasm_bindgen::JsCast;

        let window = web_sys::window().expect("Couldn't get window.");
        let document = window.document().expect("Couldn't get document.");

        let sslist = document.style_sheets();

        for index in 0..sslist.length() {
            let sheet = sslist.get(index).unwrap();
            if sheet.owner_node().unwrap().node_name() == "STYLE" {
                return sheet.dyn_into::<web_sys::CssStyleSheet>().ok();
            }
        }
        None
    }

    fn make_css_stylesheet() -> Option<CssStyleSheet> {
        use wasm_bindgen::JsCast;

        let window = web_sys::window().expect("Couldn't get window.");
        let document = window.document().expect("Couldn't get document.");

        let stylesheet = document
            .create_element("style")
            .expect("Couldn't create 'style' element")
            .dyn_into::<web_sys::HtmlStyleElement>()
            .expect("Couldn't convert new element into stylesheet.");
        stylesheet.set_type("text/css");

        let head = document.head().expect("Couldn't get head");
        head.append_child(
            &stylesheet
                .dyn_into::<web_sys::Node>()
                .expect("Couldn't convert CSS stylesheet into node"),
        );

        // Now that we've added the new css styleshneet, we find it. Yes, weird.
        CssMunger::find_css_stylesheet()
    }
}
