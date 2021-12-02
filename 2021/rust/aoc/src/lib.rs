use std::fs::read_dir;
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_registry(_item: TokenStream) -> TokenStream {
    let mut code = String::new();
    let mut registry = String::new();
    for f in read_dir("src").unwrap() {
        let f = f.unwrap().file_name().into_string().unwrap();
        if f.starts_with("day") {
            let f = f.split_once(".").unwrap().0;
            code.push_str(&format!("mod {};", f));
            registry.push_str(&format!(
                "coll.insert(\"{0}\", Box::new(|d: String, e: Option<(&str, &str)>| {0}::Day{{}}.run(&d, e)) as Box<dyn Fn(String, Option<(&str, &str)>)+ 'static>);\n",
                f
            ));
        }
    }

    code.push_str(&format!(
        r#"
thread_local! {{
    pub static DAYS: std::collections::BTreeMap<&'static str, Box<dyn Fn(String, Option<(&str, &str)>)>> = {{
        let mut coll = std::collections::BTreeMap::new();
        {}
        coll
    }}
}}"#,
        registry,
    ));
    code.parse().unwrap()
}
