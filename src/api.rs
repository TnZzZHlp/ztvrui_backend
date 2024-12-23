use salvo::prelude::*;

/// Verify Cookie legitimacy
#[handler]
pub async fn auth(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl) {
    todo!("Implement auth")
}

/// Login API
#[handler]
pub async fn login(res: &mut Response) {
    todo!("Implement login")
}

/// Logout API
#[handler]
pub async fn logout(res: &mut Response) {
    todo!("Implement logout")
}
