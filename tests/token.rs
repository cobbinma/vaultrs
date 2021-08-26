mod common;

use common::VaultServer;
use vaultrs::{api::token::requests::CreateTokenRequest, error::ClientError, token};

#[tokio::test]
async fn test() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);
    let token = setup(&server).await.unwrap();

    test_new(&server).await;
    test_new_orphan(&server).await;
    test_lookup(&server, token.token.as_str()).await;
    test_lookup_self(&server).await;
    test_lookup_accessor(&server, token.accessor.as_str()).await;
    test_renew(&server, token.token.as_str()).await;
}

pub async fn test_lookup(server: &VaultServer<'_>, token: &str) {
    let resp = token::lookup(&server.client, token).await;
    assert!(resp.is_ok());
}

pub async fn test_lookup_accessor(server: &VaultServer<'_>, accessor: &str) {
    let resp = token::lookup_accessor(&server.client, accessor).await;
    assert!(resp.is_ok());
}

pub async fn test_lookup_self(server: &VaultServer<'_>) {
    let resp = token::lookup_self(&server.client).await;
    assert!(resp.is_ok());
}

pub async fn test_new(server: &VaultServer<'_>) {
    let resp = token::new(&server.client, None).await;
    assert!(resp.is_ok());
}

pub async fn test_new_orphan(server: &VaultServer<'_>) {
    let resp = token::new_orphan(&server.client, None).await;
    assert!(resp.is_ok());
}

pub async fn test_renew(server: &VaultServer<'_>, token: &str) {
    let resp = token::renew(&server.client, token, Some("20m")).await;
    assert!(resp.is_ok());
}

// TODO: Add test for create token with role

struct Token {
    pub accessor: String,
    pub token: String,
}

async fn setup(server: &VaultServer<'_>) -> Result<Token, ClientError> {
    // Create a new token
    let resp = token::new(
        &server.client,
        Some(
            CreateTokenRequest::builder()
                .ttl("10m")
                .renewable(true)
                .explicit_max_ttl("1h"),
        ),
    )
    .await?;
    Ok(Token {
        accessor: resp.accessor,
        token: resp.client_token,
    })
}
