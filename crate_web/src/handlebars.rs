use handlebars::Handlebars;
use rocket::response::content::RawHtml;
use std::collections::HashMap;
use std::fs;

pub fn get_template(template: String, data: Option<HashMap<String, String>>) -> RawHtml<String> {
    let template_content = match fs::read_to_string(format!("crate_web/static/{}.html", template)) {
        Ok(content) => content,
        Err(_) => String::from("Error reading HTML file"), // Replace with proper error handling
    };

    let mut reg = Handlebars::new();
    reg.register_template_string("prostě", template_content)
        .unwrap();

    let data = match data {
        Some(d) => d,
        None => HashMap::new(),
    };

    match reg.render("prostě", &data) {
        Ok(page) => RawHtml(page),
        Err(e) => RawHtml(e.to_string()),
    }
}
