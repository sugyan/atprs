use super::*;
use crate::agent::store::MemorySessionStore;
use futures::future::join_all;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default)]
struct DummyResponses {
    create_session: Option<crate::com::atproto::server::create_session::Output>,
    get_session: Option<crate::com::atproto::server::get_session::Output>,
}

#[derive(Default)]
struct DummyClient {
    responses: DummyResponses,
    counts: Arc<RwLock<HashMap<String, usize>>>,
}

#[async_trait]
impl HttpClient for DummyClient {
    async fn send_http(
        &self,
        request: Request<Vec<u8>>,
    ) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        tokio::time::sleep(std::time::Duration::from_micros(10)).await;
        let builder = Response::builder().header(http::header::CONTENT_TYPE, "application/json");
        let token = request
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(' ').last());
        if token == Some("expired") {
            return Ok(builder
                .status(http::StatusCode::BAD_REQUEST)
                .body(serde_json::to_vec(
                    &atrium_xrpc::error::ErrorResponseBody {
                        error: Some(String::from("ExpiredToken")),
                        message: Some(String::from("Token has expired")),
                    },
                )?)?);
        }
        let mut body = Vec::new();
        if let Some(nsid) = request.uri().path().strip_prefix("/xrpc/") {
            *self.counts.write().await.entry(nsid.into()).or_default() += 1;
            match nsid {
                "com.atproto.server.createSession" => {
                    if let Some(output) = &self.responses.create_session {
                        body.extend(serde_json::to_vec(output)?);
                    }
                }
                "com.atproto.server.getSession" => {
                    if token == Some("access") {
                        if let Some(output) = &self.responses.get_session {
                            body.extend(serde_json::to_vec(output)?);
                        }
                    }
                }
                "com.atproto.server.refreshSession" => {
                    if token == Some("refresh") {
                        body.extend(serde_json::to_vec(
                            &crate::com::atproto::server::refresh_session::Output {
                                access_jwt: String::from("access"),
                                did: String::from("did"),
                                did_doc: None,
                                handle: String::from("handle"),
                                refresh_jwt: String::from("refresh"),
                            },
                        )?);
                    }
                }
                _ => {}
            }
        }
        if body.is_empty() {
            Ok(builder
                .status(http::StatusCode::UNAUTHORIZED)
                .body(serde_json::to_vec(
                    &atrium_xrpc::error::ErrorResponseBody {
                        error: Some(String::from("AuthenticationRequired")),
                        message: Some(String::from("Invalid identifier or password")),
                    },
                )?)?)
        } else {
            Ok(builder.status(http::StatusCode::OK).body(body)?)
        }
    }
}

impl XrpcClient for DummyClient {
    fn base_uri(&self) -> &str {
        "http://localhost:8080"
    }
}

fn session() -> Session {
    Session {
        access_jwt: String::from("access"),
        did: String::from("did"),
        did_doc: None,
        email: None,
        email_confirmed: None,
        handle: String::from("handle"),
        refresh_jwt: String::from("refresh"),
    }
}

#[tokio::test]
async fn test_new() {
    let agent = AtpAgent::new(DummyClient::default(), MemorySessionStore::default());
    assert_eq!(agent.get_session().await, None);
}

#[tokio::test]
async fn test_login() {
    let session = session();
    // success
    {
        let client = DummyClient {
            responses: DummyResponses {
                create_session: Some(crate::com::atproto::server::create_session::Output {
                    ..session.clone()
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        let agent = AtpAgent::new(client, MemorySessionStore::default());
        agent
            .login("test", "pass")
            .await
            .expect("login should be succeeded");
        assert_eq!(agent.get_session().await, Some(session));
    }
    // failure with `createSession` error
    {
        let client = DummyClient {
            responses: DummyResponses {
                ..Default::default()
            },
            ..Default::default()
        };
        let agent = AtpAgent::new(client, MemorySessionStore::default());
        agent
            .login("test", "bad")
            .await
            .expect_err("login should be failed");
        assert_eq!(agent.get_session().await, None);
    }
}

#[tokio::test]
async fn test_xrpc_get_session() {
    let session = session();
    let client = DummyClient {
        responses: DummyResponses {
            get_session: Some(crate::com::atproto::server::get_session::Output {
                did: session.did.clone(),
                did_doc: session.did_doc.clone(),
                email: session.email.clone(),
                email_confirmed: session.email_confirmed,
                handle: session.handle.clone(),
            }),
            ..Default::default()
        },
        ..Default::default()
    };
    let agent = AtpAgent::new(client, MemorySessionStore::default());
    agent.store.set_session(session).await;
    let output = agent
        .api
        .com
        .atproto
        .server
        .get_session()
        .await
        .expect("get session should be succeeded");
    assert_eq!(output.did, "did");
}

#[tokio::test]
async fn test_xrpc_get_session_with_refresh() {
    let mut session = session();
    session.access_jwt = String::from("expired");
    let client = DummyClient {
        responses: DummyResponses {
            get_session: Some(crate::com::atproto::server::get_session::Output {
                did: session.did.clone(),
                did_doc: session.did_doc.clone(),
                email: session.email.clone(),
                email_confirmed: session.email_confirmed,
                handle: session.handle.clone(),
            }),
            ..Default::default()
        },
        ..Default::default()
    };
    let agent = AtpAgent::new(client, MemorySessionStore::default());
    agent.store.set_session(session).await;
    let output = agent
        .api
        .com
        .atproto
        .server
        .get_session()
        .await
        .expect("get session should be succeeded");
    assert_eq!(output.did, "did");
    assert_eq!(
        agent.get_session().await.map(|session| session.access_jwt),
        Some("access".into())
    );
}

#[tokio::test]
async fn test_xrpc_get_session_with_duplicated_refresh() {
    let mut session = session();
    session.access_jwt = String::from("expired");
    let client = DummyClient {
        responses: DummyResponses {
            get_session: Some(crate::com::atproto::server::get_session::Output {
                did: session.did.clone(),
                did_doc: session.did_doc.clone(),
                email: session.email.clone(),
                email_confirmed: session.email_confirmed,
                handle: session.handle.clone(),
            }),
            ..Default::default()
        },
        ..Default::default()
    };
    let counts = Arc::clone(&client.counts);
    let agent = Arc::new(AtpAgent::new(client, MemorySessionStore::default()));
    agent.store.set_session(session).await;
    let handles = (0..3).map(|_| {
        let agent = Arc::clone(&agent);
        tokio::spawn(async move { agent.api.com.atproto.server.get_session().await })
    });
    let results = join_all(handles).await;
    for result in &results {
        let output = result
            .as_ref()
            .expect("task should be successfully executed")
            .as_ref()
            .expect("get session should be succeeded");
        assert_eq!(output.did, "did");
    }
    assert_eq!(
        agent.get_session().await.map(|session| session.access_jwt),
        Some("access".into())
    );
    assert_eq!(
        counts.read().await.clone(),
        HashMap::from_iter([
            ("com.atproto.server.refreshSession".into(), 1),
            ("com.atproto.server.getSession".into(), 3)
        ])
    );
}

#[tokio::test]
async fn test_resume_session() {
    let session = session();
    // success
    {
        let client = DummyClient {
            responses: DummyResponses {
                get_session: Some(crate::com::atproto::server::get_session::Output {
                    did: session.did.clone(),
                    did_doc: session.did_doc.clone(),
                    email: session.email.clone(),
                    email_confirmed: session.email_confirmed,
                    handle: session.handle.clone(),
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        let agent = AtpAgent::new(client, MemorySessionStore::default());
        assert_eq!(agent.get_session().await, None);
        agent
            .resume_session(Session {
                email: Some(String::from("test@example.com")),
                ..session.clone()
            })
            .await
            .expect("resume_session should be succeeded");
        assert_eq!(agent.get_session().await, Some(session.clone()));
    }
    // failure with `getSession` error
    {
        let client = DummyClient {
            responses: DummyResponses {
                ..Default::default()
            },
            ..Default::default()
        };
        let agent = AtpAgent::new(client, MemorySessionStore::default());
        assert_eq!(agent.get_session().await, None);
        agent
            .resume_session(session)
            .await
            .expect_err("resume_session should be failed");
        assert_eq!(agent.get_session().await, None);
    }
}

#[tokio::test]
async fn test_resume_session_with_refresh() {
    let session = session();
    let client = DummyClient {
        responses: DummyResponses {
            get_session: Some(crate::com::atproto::server::get_session::Output {
                did: session.did.clone(),
                did_doc: session.did_doc.clone(),
                email: session.email.clone(),
                email_confirmed: session.email_confirmed,
                handle: session.handle.clone(),
            }),
            ..Default::default()
        },
        ..Default::default()
    };
    let agent = AtpAgent::new(client, MemorySessionStore::default());
    agent
        .resume_session(Session {
            access_jwt: "expired".into(),
            ..session.clone()
        })
        .await
        .expect("resume_session should be succeeded");
    assert_eq!(agent.get_session().await, Some(session));
}
