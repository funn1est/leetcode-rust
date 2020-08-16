use std::fs;
use std::io::Write;
use std::path::Path;

use handlebars::Handlebars;
use regex::Regex;
use serde::Serialize;

use crate::errors::{LcError, Result};
use crate::services::question::get_question_data;
use crate::types::query::CodeDefinition;

#[derive(Serialize)]
struct Params {
    code: String,
    question_name: String,
    fn_name: String,
}

pub struct LcCli {}

impl LcCli {
    #[tokio::main]
    pub async fn get(question_name: String) -> Result<()> {
        let question_data = get_question_data(&question_name).await?;
        let codes: Vec<CodeDefinition> =
            serde_json::from_str(&question_data.data.question.code_definition)?;
        let rust_code = &codes
            .iter()
            .find(|&d| d.value == String::from("rust"))
            .ok_or(LcError::NoRustCode)?
            .default_code;
        let re = Regex::new(r"(?m)^(?:.* pub fn )(\w*)")?;
        let fn_name = re
            .captures(rust_code)
            .ok_or_else(|| "no capture")?
            .get(1)
            .ok_or_else(|| "no match")?
            .as_str()
            .to_owned();

        let handlebars = {
            let mut h = Handlebars::new();
            h.register_escape_fn(handlebars::no_escape);
            h
        };

        let template = fs::read_to_string("./templates/solution.hbs")?;

        let params = Params {
            code: String::from(rust_code),
            question_name: question_name.to_owned(),
            fn_name,
        };

        let question_id = question_data.data.question.question_frontend_id;
        let file_text = handlebars.render_template(template.as_str(), &params)?;
        let file_name = format!("n{}_{}", question_id, question_name.replace("-", "_"));
        let file_dir = format!("src/solutions/{}", file_name);
        fs::create_dir_all(Path::new(&file_dir))?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Path::new(&format!("{}/mod.rs", file_dir)))?;
        file.write_all(file_text.as_bytes())?;

        let mut lib_file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("./src/solutions/mod.rs")
            .unwrap();
        writeln!(lib_file, "pub mod {};", file_name)?;
        Ok(())
    }
}
