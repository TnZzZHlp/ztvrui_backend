use include_dir::{ include_dir, Dir };
use salvo::prelude::*;

static FRONTEND: Dir = include_dir!("./dist");

#[handler]
pub async fn index(req: &mut Request, res: &mut Response) {
    let path = req.uri().path();

    todo!()
}

fn get_file(file: String) -> Result<Vec<u8>, ()> {
    todo!()
}
