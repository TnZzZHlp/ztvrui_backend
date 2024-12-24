use salvo::{ http::cookie::Cookie, prelude::* };
use serde_json::json;

use crate::DB;

/// Verify Cookie legitimacy
#[handler]
pub async fn auth(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl) {
    let refuse = |res: &mut Response, ctrl: &mut FlowCtrl| {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(
            Json(
                json!({
                "path": "/",
                "error": "No cookie found"
            })
            )
        );
        ctrl.skip_rest();
    };

    let cookie = match req.cookie("Token") {
        Some(cookie) => cookie.value(),
        None => {
            refuse(res, ctrl);
            return;
        }
    };

    if !DB.verify_cookie(cookie).await {
        refuse(res, ctrl);
        return;
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
            res.render(Json(json!({
                "error": "Invalid request"
            })));
            ctrl.skip_rest();
            return;
        }
    };

    if DB.verify(username, password).await {
        let cookie = uuid::Uuid::new_v4().to_string();

        DB.update_user_cookie(username, &cookie).await.unwrap();

        res.add_header(
            "Set-Cookie",
            Cookie::build(("Token", cookie)).path("/").permanent().build().to_string(),
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
pub async fn logout(req: &mut Request, res: &mut Response) {
    let cookie = req.cookie("Token").unwrap().value();

    DB.remove_cookie(cookie).await.unwrap();

    res.render(Json(json!({
        "error": "0"
    })));
}

/// Check Login Status
#[handler]
pub async fn check(res: &mut Response, _req: &mut Request) {
    res.render(Json(json!({
        "error": "0"
    })));
}
