use anyhow::Result;
use async_trait::async_trait;
use tonic::{transport::Server, Request, Response, Status};
pub mod proto {
  tonic::include_proto!("proto");
}

use proto::rmw_server::{Rmw, RmwServer};
use proto::{HiIn, HiOut};

#[derive(Default)]
pub struct RmwSrv {}

#[async_trait]
impl Rmw for RmwSrv {
  async fn hi(&self, request: Request<HiIn>) -> Result<Response<HiOut>, Status> {
    println!("Got a request from {:?}", request.remote_addr());

    let reply = HiOut {
      message: format!("Hello {}!", request.into_inner().name),
    };
    Ok(Response::new(reply))
  }
}

pub async fn listen(addr: String) -> Result<()> {
  let rmw = RmwServer::new(RmwSrv::default());

  Server::builder()
    .accept_http1(true)
    .add_service(tonic_web::enable(rmw))
    .serve(addr.parse()?)
    .await?;

  Ok(())
}
