use super::models::tenant_grpc::TenantResponse;
use crate::domain::TenantBO;

pub fn tenant_bo_to_tenant_response(tenant: &TenantBO) -> TenantResponse {
    let response = TenantResponse {
        id: tenant.id.clone(),
        name: tenant.name.clone(),
    };
    response
}
