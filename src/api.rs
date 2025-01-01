use base64::prelude::*;
use salvo::{ http::cookie::Cookie, prelude::* };
use serde_json::json;
use std::time::{ SystemTime, UNIX_EPOCH };

use crate::{ CONFIG, COOKIE };

/// Verify Cookie legitimacy
#[handler]
pub async fn auth(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl) {
    let refuse = |res: &mut Response, ctrl: &mut FlowCtrl| {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(
            Json(json!({
            "path": "/",
            "error": "No cookie found"
        }))
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

    if *COOKIE.read().await != cookie {
        refuse(res, ctrl);
        return;
    }
}

/// Login API
#[handler]
pub async fn login(res: &mut Response, req: &mut Request, ctrl: &mut FlowCtrl) {
    let body = req.parse_json::<serde_json::Value>().await.unwrap();

    let (username, password) = match (body.get("username"), body.get("password")) {
        (Some(username), Some(password)) => {
            (username.as_str().unwrap(), password.as_str().unwrap())
        }
        _ => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({
                "error": "Invalid request"
            })));
            ctrl.skip_rest();
            return;
        }
    };

    if CONFIG.read().await.verify(username, password).await {
        let cookie = BASE64_STANDARD.encode(
            format!(
                "{}:{}",
                uuid::Uuid::new_v4(),
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
            ).as_bytes()
        );

        res.add_header(
            "Set-Cookie",
            Cookie::build(("Token", &cookie)).path("/").permanent().build().to_string(),
            true
        ).unwrap();

        res.render(Json(json!({
            "error": "0"
        })));

        *COOKIE.write().await = cookie;
    } else {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(Json(json!({
            "error": "invalid_username_or_password"
        })));
    }
}

/// Logout API
#[handler]
pub async fn logout(_req: &mut Request, res: &mut Response) {
    *COOKIE.write().await = String::new();

    res.render(Json(json!({
        "error": "0"
    })));
}

/// Modify Username Or Password API
#[handler]
pub async fn modify(res: &mut Response, req: &mut Request) {
    let body = req.parse_json::<serde_json::Value>().await.unwrap();

    let (username, password) = match (body.get("username"), body.get("password")) {
        (Some(username), Some(password)) => {
            (username.as_str().unwrap(), password.as_str().unwrap())
        }
        _ => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(json!({
                "error": "Invalid request"
            })));
            return;
        }
    };

    {
        CONFIG.write().await.update_user_info(username, password).await;
    }

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
