use std::sync::Arc;
use crate::domain::repositories::todo::TodoRepository;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::service_context::ServiceContextService;
use crate::domain::services::todo::TodoService;
use crate::domain::services::user::UserService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::todo::TodoDieselRepository;
use crate::infrastructure::repositories::user::UserDieselRepository;
use crate::infrastructure::services::service_context::ServiceContextServiceImpl;
use crate::services::todo::TodoServiceImpl;
use crate::services::user::UserServiceImpl;

pub struct Container {
    pub todo_service: Arc<dyn TodoService>,
    pub service_context_service: Arc<dyn ServiceContextService>,
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    pub fn new() -> Self {
        let db_pool = db_pool();
        let todo_repository: Arc<dyn TodoRepository> = Arc::new(
            TodoDieselRepository::new(Arc::new(db_pool.clone()))
        );
        let todo_service = Arc::new(
            TodoServiceImpl { repository: todo_repository }
        );
        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserDieselRepository::new(Arc::new(db_pool.clone()))
        );
        let user_service = Arc::new(
            UserServiceImpl { repository: user_repository }
        );
        let service_context_service = Arc::new(
            ServiceContextServiceImpl::new(Arc::new(db_pool.clone()))
        );

        Container { todo_service, service_context_service, user_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
