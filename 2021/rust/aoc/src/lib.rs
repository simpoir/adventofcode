extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_registry(_item: TokenStream) -> TokenStream {
    let mut code = String::new();
    let mut registry = String::new();
    for i in 1..31 {
        if std::path::Path::new(&format!("src/day{}.rs", i)).exists() {
            let f = format!("day{}", i);
            code.push_str(&format!("mod {};", f));
            registry.push_str(&format!(
                "coll.push((\"{0}\", Box::new(|d: String, e: Option<(&str, &str)>| {0}::Day{{}}.run(&d, e)) as Box<dyn Fn(String, Option<(&str, &str)>)+ 'static>));\n",
                f
            ));
        }
    }

    code.push_str(&format!(
        r#"
thread_local! {{
    pub static DAYS: Vec<(&'static str, Box<dyn Fn(String, Option<(&str, &str)>)>)> = {{
        let mut coll = vec![];
        {}
        coll
    }}
}}"#,
        registry,
    ));
    code.parse().unwrap()
}
