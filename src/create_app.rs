use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::Error;
use actix_web::{web, App};

use crate::api::controllers::user_handler::{
    create_user_handler, login_user_handler, validate_token_handler,
};
use crate::api::middleware::ServiceContextMaintenanceCheck;
use crate::container::Container;

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let container = Container::new();
    let user_service = container.user_service.clone();
    // the last
    let service_context_service = container.service_context_service.clone();
    App::new()
        .app_data(web::Data::from(user_service.clone()))
        .app_data(web::Data::from(service_context_service.clone()))
        .wrap(Logger::default())
        .wrap(ServiceContextMaintenanceCheck)
        .service(
            web::scope("/auth")
                .route("/register", web::post().to(create_user_handler))
                .route("/login", web::post().to(login_user_handler))
                .route("/validate", web::post().to(validate_token_handler)),
        )
}
