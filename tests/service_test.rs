use activ::config::{get_configuration, DatabaseSettings};
use activ::startup::run;
use sqlx::{Connection, PgConnection, PgPool, Executor};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub addr: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let addr = format!("http://127.0.0.1:{}", port);

    let mut config = get_configuration().expect("Failed to read config");
    config.database.database_name = Uuid::new_v4().to_string();

    let db_pool = configure_db(&config.database).await;

    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { addr, db_pool }
}

async fn configure_db(config: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    conn
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database");
    
    let db_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the db");

    db_pool
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn home_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/", &app.addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let params = serde_json::json!({
        "name": "full_name",
        "email": "algo@mail.com"
    });

    let response = client
        .post(&format!("{}/subscriptions", &app.addr))
        .json(&params)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, params["email"]);
    assert_eq!(saved.name, params["name"]);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_invalid_data(){
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=Pedro", "missing the email"),
        ("email=endereco%40mail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (body, msg) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");
        
        assert_eq!(400, response.status().as_u16(), "{}", msg);
    };
}