use crate::domain::{self};
use crate::grpc::tenant_mapper;
use function_name::named;
use log::debug;
use opentelemetry::{
    global,
    propagation::Extractor,
    trace::{Span, SpanKind, Tracer},
};
use std::sync::Arc;

use super::models::tenant_grpc::tenant_grpc_server::TenantGrpc;
use super::models::tenant_grpc::tenant_grpc_server::TenantGrpcServer;
use super::models::tenant_grpc::{
    CreateTenantRequest, DeleteTenantRequest, DeleteTenantResponse, GetTenantListRequest,
    GetTenantRequest, TenantListResponse, TenantResponse, UpdateTenantRequest,
};
use tonic::{Request, Response, Status};

pub fn create_grpc_service() -> TenantGrpcServer<TenantGrpcService> {
    let domain_tenant_service = domain::tenant_service::TenantService {};
    let my_tenant_service = TenantGrpcService {
        tenant_service: Arc::new(domain_tenant_service),
    };
    return TenantGrpcServer::new(my_tenant_service);
}
// Implement the service skeleton for the "Tenant" service
// defined in the proto
pub struct TenantGrpcService {
    tenant_service: Arc<dyn domain::ITenantService>,
}

fn span_creator<T>(name: String, req: &Request<T>) -> global::BoxedSpan
where
    T: std::fmt::Debug,
{
    let parent_cx =
        global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(req.metadata())));
    let tracer = global::tracer("rust/example");
    let mut span = tracer
        .span_builder(name)
        .with_kind(SpanKind::Server)
        .start_with_context(&tracer, &parent_cx);

    span.add_event(format!("Received Request: {:?}", req.get_ref()), vec![]);

    span
}

// Implement the service function(s) defined in the proto
// for the Tenant service
#[tonic::async_trait]
impl TenantGrpc for TenantGrpcService {
    #[named]
    async fn get_tenant(
        &self,
        request: Request<GetTenantRequest>,
    ) -> Result<Response<TenantResponse>, Status> {
        debug!("call getTenant ...");

        // Create a span for the request (operntelemetry span)
        let mut span = span_creator(function_name!().to_string(), &request);
        span.add_event("test event", vec![]);

        let tenant = self.tenant_service.get_tenant();
        let response = tenant_mapper::tenant_bo_to_tenant_response(&tenant);

        Ok(Response::new(response))
    }

    #[named]
    async fn get_tenant_list(
        &self,
        request: Request<GetTenantListRequest>,
    ) -> Result<Response<TenantListResponse>, Status> {
        debug!("call getTenantList ...");

        // Create a span for the request (operntelemetry span)
        let _ = span_creator(function_name!().to_string(), &request);

        let tenant_list = self.tenant_service.get_tenant_list();
        let response = TenantListResponse {
            tenants: tenant_list
                .iter()
                .map(|tenant| tenant_mapper::tenant_bo_to_tenant_response(tenant))
                .collect(),
        };

        Ok(Response::new(response))
    }

    #[named]
    async fn create_tenant(
        &self,
        request: Request<CreateTenantRequest>,
    ) -> Result<Response<TenantResponse>, Status> {
        debug!("call createTenant ...");

        // Create a span for the request (operntelemetry span)
        let _ = span_creator(function_name!().to_string(), &request);

        let tenant = self.tenant_service.create_tenant();
        let response = tenant_mapper::tenant_bo_to_tenant_response(&tenant);
        Ok(Response::new(response))
    }

    #[named]
    async fn update_tenant(
        &self,
        request: Request<UpdateTenantRequest>,
    ) -> Result<Response<TenantResponse>, Status> {
        debug!("call updateTenant ...");

        // Create a span for the request (operntelemetry span)
        let _ = span_creator(function_name!().to_string(), &request);

        let tenant = self.tenant_service.update_tenant();
        let response = tenant_mapper::tenant_bo_to_tenant_response(&tenant);
        Ok(Response::new(response))
    }

    #[named]
    async fn delete_tenant(
        &self,
        request: Request<DeleteTenantRequest>,
    ) -> Result<Response<DeleteTenantResponse>, Status> {
        debug!("call deleteTenant ...");

        // Create a span for the request (operntelemetry span)
        let _ = span_creator(function_name!().to_string(), &request);

        self.tenant_service.delete_tenant();
        let response = DeleteTenantResponse {};
        Ok(Response::new(response))
    }
}
struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);

impl<'a> Extractor for MetadataMap<'a> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}
