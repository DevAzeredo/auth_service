use crate::domain::repositories::user::UserRepository;
use crate::domain::services::service_context::ServiceContextService;
use crate::domain::services::user::UserService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::user::UserDieselRepository;
use crate::infrastructure::services::service_context::ServiceContextServiceImpl;
use crate::services::token::TokenServiceImpl;
use crate::services::user::UserServiceImpl;
use std::sync::Arc;

pub struct Container {
    pub service_context_service: Arc<dyn ServiceContextService>,
    pub user_service: Arc<dyn UserService>,
}
impl Container {
    pub fn new() -> Self {
        let db_pool = db_pool();
        let user_repository: Arc<dyn UserRepository> =
            Arc::new(UserDieselRepository::new(Arc::new(db_pool.clone())));
        let token_service = Arc::new(TokenServiceImpl {});
        let user_service = Arc::new(UserServiceImpl {
            repository: user_repository,
            token_service,
        });
        let service_context_service =
            Arc::new(ServiceContextServiceImpl::new(Arc::new(db_pool.clone())));
        Container {
            service_context_service,
            user_service,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
