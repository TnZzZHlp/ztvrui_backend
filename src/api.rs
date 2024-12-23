use salvo::{ http::cookie::Cookie, prelude::* };
use serde_json::json;

use crate::DB;

/// Verify Cookie legitimacy
#[handler]
pub async fn auth(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl) {
    let cookie = match req.cookie("Token") {
        Some(cookie) => cookie.value(),
        None => {
            res.status_code(StatusCode::UNAUTHORIZED);
            ctrl.skip_rest();
            return;
        }
    };

    if !DB.verify_cookie(cookie).await {
        res.status_code(StatusCode::UNAUTHORIZED);
        ctrl.skip_rest();
    }
}

/// Login API
#[handler]
pub async fn login(res: &mut Response, req: &mut Request, ctrl: &mut FlowCtrl) {
    let body = req.parse_json::<serde_json::Value>().await.unwrap();

    let (username, password) = match (body.get("username"), body.get("password")) {
        (Some(username), Some(password)) =>
            (username.as_str().unwrap(), password.as_str().unwrap()),
        _ => {
            res.status_code(StatusCode::BAD_REQUEST);
            ctrl.skip_rest();
            return;
        }
    };

    if DB.verify(username, password).await {
        let cookie = uuid::Uuid::new_v4().to_string();

        DB.update_user_cookie(username, &cookie).await.unwrap();

        res.add_header(
            "Set-Cookie",
            Cookie::build(("Token", cookie)).build().to_string(),
            true
        ).unwrap();

        res.render(Json(json!({
            "error": "0"
        })));
    } else {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(Json(json!({
            "error": "Invalid username or password"
        })));
    }
}

/// Logout API
#[handler]
pub async fn logout(res: &mut Response) {
    todo!("Implement logout")
}
