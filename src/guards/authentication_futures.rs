use actix_web::Error;
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use futures::{ready, Future};
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::marker::PhantomData;

/// AuthenticationFuture struct
#[pin_project]
pub struct AuthenticationFuture<S, B> where S: Service<ServiceRequest>, {
    #[pin]
    pub fut: S::Future,
    pub _phantom: PhantomData<B>,
}

/// Implement Future for AuthenticationFuture
impl<S, B> Future for AuthenticationFuture<S, B>
    where
        B: MessageBody,
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Output = Result<ServiceResponse<EitherBody<B>>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = match ready!(self.project().fut.poll(cx)) {
            Ok(res) => res,
            Err(err) => return Poll::Ready(Err(err.into())),
        };

        Poll::Ready(Ok(res.map_into_left_body()))
    }
}