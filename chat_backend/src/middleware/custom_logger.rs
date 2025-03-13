use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::time::Instant;

pub struct CustomLogger;

impl<S, B> Transform<S, ServiceRequest> for CustomLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CustomLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CustomLoggerMiddleware { service })
    }
}

pub struct CustomLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CustomLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let uri = req.uri().clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let status = res.response().status();
            let elapsed = start.elapsed().as_millis();

            // Log based on status code
            if status.is_success() {
                log::info!("{} {} -> {} ({}ms)", method, uri, status, elapsed);
            } else {
                log::error!("{} {} -> {} ({}ms)", method, uri, status, elapsed);
            }

            Ok(res)
        })
    }
}
