use crate::model::Auth;

use actix_web::{web, HttpResponse};

pub fn authenticate_credentials(Auth { .. }: Auth) {
    //    if let Some(auth_record) = user::by_name(&name) {
    //        match password::authenticate(&password, &auth_record.hash) {
    //            Ok(valid) => {
    //                if valid {
    //                    let token = jwt::generate_token(auth_record);
    //                    HttpResponse::Ok().header(token);
    //                } else {
    //                    HttpResponse::Unauthorized().body("Invalid Credentials");
    //                }
    //            }
    //            Err(error) => panic!(error),
    //        };
    //    } else {
    //    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("").route(web::get().to(|| HttpResponse::Ok().body("do some stupid stuff!"))),
    );
}
