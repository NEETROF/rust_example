use crate::domain::{ITenantService, TenantBO};

pub struct TenantService {}

impl ITenantService for TenantService {
    fn get_tenant(&self) -> TenantBO {
        let tenant = TenantBO {
            id: String::from("1"),
            name: String::from("name 1"),
        };
        tenant
    }

    fn get_tenant_list(&self) -> Vec<TenantBO> {
        let mut tenant_list = Vec::new();
        for i in 0..=10 {
            let tenant = TenantBO {
                id: i.to_string(),
                name: format!("name {}", i),
            };
            tenant_list.push(tenant);
        }
        tenant_list
    }

    fn create_tenant(&self) -> TenantBO {
        let tenant = TenantBO {
            id: String::from("1"),
            name: String::from("name 1"),
        };
        tenant
    }

    fn update_tenant(&self) -> TenantBO {
        let tenant = TenantBO {
            id: String::from("1"),
            name: String::from("name 1"),
        };
        tenant
    }

    fn delete_tenant(&self) {}
}
