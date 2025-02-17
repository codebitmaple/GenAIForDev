use actix_service::{Service, Transform};
use actix_session::SessionExt;
use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use futures::FutureExt;
use log::{debug, info};

use crate::shared::ops::{
    environ_ops::Environment,
    jwt_ops::{get_claims_from, validate_jwt},
};
use crate::shared::{
    auth::user::UserAuth,
    ops::environ_ops::{Environ, WebConfig},
};

pub struct LoginInterceptor;

impl<S> Transform<S, ServiceRequest> for LoginInterceptor
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(
        &self,
        service: S,
    ) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(
        &self,
        service_req: ServiceRequest,
    ) -> Self::Future {
        info!("LoginVerifier");
        // let http_req = service_req.head().clone();
        let session = service_req.get_session();
        let user_auth = UserAuth::from(session);
        let web_config: WebConfig = Environ::init();
        match Environment::get_env() {
            Environment::Dev => {
                if web_config.allow_debug {
                    debug!("Dev environment BUT debug is allowed, proceeding with request");
                    self.service.call(service_req).boxed_local()
                } else {
                    debug!("Dev environment BUT debug is not allowed, redirecting to login");
                    redirect_to(service_req, "/login")
                }
            }
            Environment::Prod => {
                match user_auth.jwt.clone() {
                    Some(jwt) => {
                        debug!("JWT found in session");
                        if validate_jwt(&jwt).is_some() {
                            // Extract claims from the token
                            debug!("Valid JWT, extracting claims");
                            if get_claims_from(&jwt).is_none() {
                                debug!("Failed to extract claims from JWT, redirecting to login");
                                return redirect_to(service_req, "/login");
                            }

                            debug!("JWT validation successful, proceeding with request");
                            self.service.call(service_req).boxed_local()
                        } else {
                            debug!("JWT validation failed, redirecting to login");
                            // JWT validation failed, redirect to login
                            redirect_to(service_req, "/login")
                        }
                    }
                    None => {
                        debug!("JWT not found in session, redirecting to login");
                        redirect_to(service_req, "/login")
                    }
                }
            }
        }
    }
}

// Function to redirect the user to the login page
fn redirect_to(
    service_req: ServiceRequest,
    location: &str,
) -> LocalBoxFuture<'static, Result<ServiceResponse<BoxBody>, Error>> {
    // Create the HTTP redirect response
    let response = HttpResponse::Found().append_header(("Location", location)).finish().map_into_boxed_body();

    // Split the ServiceRequest into parts to create the ServiceResponse
    let (request, _payload) = service_req.into_parts();
    let service_response = ServiceResponse::new(request, response);

    // Return the ServiceResponse in an async block
    async { Ok(service_response) }.boxed_local()
}
