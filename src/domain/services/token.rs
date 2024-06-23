use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::token::Claim;

#[async_trait]
pub trait TokenService: Sync + Send {
    /// Cria um token JWT para um usuário identificado pelo `user_id`.
    ///
    /// # Parâmetros
    /// - `user_id`: ID único do usuário para o qual o token JWT será criado.
    ///
    /// # Retornos
    /// - `Result<String, CommonError>`: Retorna o token JWT como uma `String` em caso de sucesso ou um `CommonError` em caso de falha.
    ///
    /// # Erros
    /// - Retorna um `CommonError` se:
    ///   - O serviço de token não conseguir gerar o token.
    ///
    /// # Exemplos
    ///
    /// ```rust
    /// use auth_service::domain::services::token::TokenService;
    ///  async fn example_usage(service: &impl TokenService) {
    ///     let user_id = 1;
    ///
    ///     match service.create(user_id).await {
    ///         Ok(token) => println!("Token JWT criado: {}", token),
    ///         Err(e) => eprintln!("Erro ao criar o token JWT: {:?}", e),
    ///     }
    /// }
    /// ```
    async fn create(&self, user_id: i32) -> Result<String, CommonError>;
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
    /// use auth_service::domain::services::token::TokenService;
    ///  async fn example_usage(service: &impl TokenService) {
    ///     let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    ///
    ///     match service.validate(token.to_string()).await {
    ///         Ok(claim) => println!("Token válido para usuário: {}", claim.sub),
    ///         Err(e) => eprintln!("Erro ao validar o token: {:?}", e),
    ///     }
    /// }
    /// ```
    async fn validate(&self, token: String) -> Result<Claim, CommonError>;
}
