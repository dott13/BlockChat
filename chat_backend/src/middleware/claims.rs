use std::rc::Rc;
use std::task::{Context, Poll};
use actix_web::guard::{Guard, GuardContext};
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};

use crate::utils::check_auth_user::AuthenticatedUser;

pub struct RoleGuard { 
    allowed_roles: Vec<String>,
}

impl RoleGuard {
    pub fn new(allowed_roles: Vec<&'static str>) -> Self {
        Self {
            allowed_roles: allowed_roles.into_iter().map(String::from).collect(),
        }
    }
}

impl Guard for RoleGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        // Extract headers from the request
        let headers = ctx.head().headers();
        
        // Try to authenticate the user from headers
        match AuthenticatedUser::from_headers_ref(headers) {
            Ok(auth_user) => {
                // Check if user's role is in allowed roles
                self.allowed_roles.contains(&auth_user.0.role)
            },
            Err(_) => false
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RoleGuard
where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static, // Ensure B can be boxed
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = RoleGuardMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RoleGuardMiddleware {
            service: Rc::new(service),
            allowed_roles: self.allowed_roles.clone(),
        })
    }
}

pub struct RoleGuardMiddleware<S> {
    service: Rc<S>,
    allowed_roles: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for RoleGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Delegate readiness to the inner service.
    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
         self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let allowed_roles = self.allowed_roles.clone();
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            match AuthenticatedUser::from_headers(req.request()) {
                Ok(auth_user) => {
                    log::debug!("Token decoded. Role: {}, Sub: {}", auth_user.0.role, auth_user.0.sub);
                    if allowed_roles.contains(&auth_user.0.role) {
                        return service.call(req)
                            .await
                            .map(|res| res.map_into_boxed_body());
                    } else {
                        log::debug!("Role {} not allowed. Allowed roles: {:?}", auth_user.0.role, allowed_roles);
                    }
                },
                Err(_) => {}
            }
            let response = HttpResponse::Unauthorized().json("Unauthorized");
            Ok(req.into_response(response))
        })
    }
}
