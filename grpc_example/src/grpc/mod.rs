use std::net::SocketAddr;

use self::{
    models::tenant_grpc::tenant_grpc_server::TenantGrpcServer,
    tenant_grpc_service::TenantGrpcService,
};

mod models;
pub mod tenant_grpc_service;
pub mod tenant_mapper;

#[derive(Debug)]
pub struct OtlpConfig {
    pub endpoint: String,
}

#[allow(dead_code)]
pub async fn start_grpc_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<TenantGrpcServer<TenantGrpcService>>()
        .await;

    tonic::transport::Server::builder()
        .add_service(tenant_grpc_service::create_grpc_service())
        .add_service(health_service)
        .serve(addr)
        .await?;
    Ok(())
}
