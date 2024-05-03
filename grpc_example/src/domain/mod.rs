pub mod tenant_service;

pub struct TenantBO {
    pub id: String,
    pub name: String,
}

pub trait ITenantService: Send + Sync + 'static {
    fn get_tenant(&self) -> TenantBO;
    fn get_tenant_list(&self) -> Vec<TenantBO>;
    fn create_tenant(&self) -> TenantBO;
    fn update_tenant(&self) -> TenantBO;
    fn delete_tenant(&self);
}
