use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::token::Claim;
use crate::domain::models::user::{CreateUser, LoginUser, User};

#[async_trait]
pub trait UserService: Sync + Send {
    /// Cria um novo usuário no sistema.
    ///
    /// # Parâmetros
    /// - `user`: Estrutura `CreateUser` contendo os dados do usuário a ser criado (incluindo a senha em texto claro).
    ///
    /// # Retornos
    /// - `Result<User, CommonError>`: Retorna o objeto `User` criado em caso de sucesso ou um `CommonError` em caso de falha.
    ///
    /// # Erros
    /// - Retorna um `CommonError` se:
    ///   - Houver um erro ao fazer o hash da senha do usuário.
    ///   - O repositório não conseguir criar o usuário.
    ///
    /// # Exemplos
    ///
    /// ```rust
    /// use auth_service::domain::models::user::CreateUser;
    /// use auth_service::domain::services::user::UserService;
    ///  async fn example_usage(service: &impl UserService) {
    ///     let new_user = CreateUser {
    ///         username: "example".to_string(),
    ///         email: "example@example.com".to_string(),
    ///         password: "password123".to_string(),
    ///     };
    ///
    ///     match service.create(new_user).await {
    ///         Ok(user) => println!("Usuário criado: {:?}", user),
    ///         Err(e) => eprintln!("Erro ao criar o usuário: {:?}", e),
    ///     }
    /// }
    /// ```
    async fn create(&self, user: CreateUser) -> Result<User, CommonError>;
    /// Gera um token JWT para um usuário autenticado.
    ///
    /// # Parâmetros
    /// - `login_user`: Estrutura `LoginUser` contendo as credenciais do usuário (nome de usuário e senha).
    ///
    /// # Retornos
    /// - `Result<String, CommonError>`: Retorna o token JWT como uma `String` em caso de sucesso ou um `CommonError` em caso de falha.
    ///
    /// # Erros
    /// - Retorna um `CommonError` se:
    ///   - Houver um erro ao fazer o hash da senha do usuário.
    ///   - O repositório não conseguir encontrar um usuário correspondente.
    ///   - O serviço de token não conseguir criar um token.
    ///
    /// # Exemplos
    ///
    /// ```rust
    /// use auth_service::domain::models::user::LoginUser;
    /// use auth_service::domain::services::user::UserService;
    ///  async fn example_usage(service: &impl UserService) {
    ///     let login_user = LoginUser {
    ///         username: "example".to_string(),
    ///         password: "password123".to_string(),
    ///     };
    ///
    ///     match service.get_token(login_user).await {
    ///         Ok(token) => println!("Token gerado: {}", token),
    ///         Err(e) => eprintln!("Erro ao gerar o token: {:?}", e),
    ///     }
    /// }
    /// ```
    async fn get_token(&self, login_user: LoginUser) -> Result<String, CommonError>;
    /// Valida um token JWT e retorna suas claims se válido.
    ///
    /// # Parâmetros
    /// - `token`: Token JWT a ser validado.
    ///
    /// # Retornos
    /// - `Result<Claim, CommonError>`: Retorna as claims decodificadas do token JWT em caso de sucesso ou um `CommonError` em caso de falha.
    ///
    /// # Erros
    /// - Retorna um `CommonError` se:
    ///   - O token JWT for inválido ou expirado.
    ///   - O serviço de token não conseguir validar o token.
    ///
    /// # Exemplos
    ///
    /// ```rust
    /// use auth_service::domain::services::user::UserService;
    ///  async fn example_usage(service: &impl UserService) {
    ///     let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    ///
    ///     match service.validate_token(token.to_string()).await {
    ///         Ok(claim) => println!("Token válido para o usuário: {}", claim.sub),
    ///         Err(e) => eprintln!("Erro ao validar o token: {:?}", e),
    ///     }
    /// }
    /// ```
    async fn validate_token(&self, token: String) -> Result<Claim, CommonError>;
}
