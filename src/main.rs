use rocket::serde::{json::Json, Deserialize};
use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct FileData<'a> {
    filename: &'a str,
    content: &'a str,
}

#[get("/<filename>")]
fn get_file(filename: &str) -> String {
    let directory = env::var("DIRECTORY").unwrap_or("/tmp/".to_string());

    match fs::read_to_string(Path::new(&format!("{directory}{filename}"))) {
        Ok(content) => {
            return format!("Successfull:\n'{content}'");
        }
        Err(why) => format!("Error reading {directory}{filename}\n{}", why.kind()),
    }
}

#[post("/new", data = "<file_data>")]
fn add_file(file_data: Json<FileData<'_>>) -> String {
    let directory = env::var("DIRECTORY").unwrap_or("/tmp/".to_string());

    match write_to_file(
        file_data.content,
        Path::new(&format!("{directory}{}", file_data.filename)),
    ) {
        Ok(_) => format!("Successful! Wrote to {directory}{}", file_data.filename),
        Err(why) => format!(
            "Error writing to {directory}{}\n{}",
            file_data.filename,
            why.kind()
        ),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_file, add_file])
}

fn write_to_file(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}
