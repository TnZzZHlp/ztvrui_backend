use include_dir::{ include_dir, Dir };
use salvo::{ handler, Request, Response };

static PROJECT_DIR: Dir = include_dir!("./dist");

#[handler]
pub async fn index(res: &mut Response, req: &mut Request) {}

fn get_file(file: String) -> Result<Vec<u8>, std::io::Error> {
    match PROJECT_DIR.get_file(file) {
        Some(file) => Ok(file.contents().to_vec()),
        None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")),
    }
}
