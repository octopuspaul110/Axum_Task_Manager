use std::{env, net::Shutdown, sync::Arc};
use axum::{Router, routing::{get, post}};
use sqlx::{postgres::PgPoolOptions};
use dotenvy::dotenv;
use tokio::signal;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};

mod error;
mod state;
mod models;
mod auth;
mod routes;


#[tokio::main]
async fn main() {
    
    dotenv().ok();

    tracing_subscriber::fmt()
    .with_env_filter(
        tracing_subscriber::EnvFilter::from_default_env()
    )
    .init();

    let database_url = env::var("DATABASE_URL")
    .expect("DTATBASE_URL must be set in .env or environment variables");

    let jwt_secret = env::var("JWT_SECRET")
    .expect("JWT_SECRET must be set in .env or environement variables");

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
    .max_connections(20)
    .connect(&database_url)
    .await
    .expect("Failed to connect to the database. Is PostgreSQL running?");
    tracing::info!("Databse connection established");

    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect(
        "Failed to run migrations. Check your migration files for SQL errors"
    );
    tracing::info!("Migrations complete");

    let state = state::AppState {
        db : pool,
        jwt_secret : Arc::new(jwt_secret)
    };

    let auth_routes = Router::new()
    .route("/register",post(auth::handlers::register))
    .route("/login", post(auth::handlers::login));

    let user_routes = Router::new()
    .route("/me", get(routes::users::get_me));

    let project_routes = Router::new()
    .route("/", 
    get(routes::projects::list_projects)
    .post(routes::projects::create_project)
    )
    .route("/{id}", 
    get(routes::projects::get_project)
                .put(routes::projects::update_project)
                .delete(routes::projects::delete_project)
            )
    .route("/{id}/tasks", 
    get(routes::tasks::list_tasks)
                   .post(routes::tasks::create_task));

    let task_routes = Router::new()
    .route("/{id}",
    get(routes::tasks::get_task)
    .put(routes::tasks::update_task)
    .delete(routes::tasks::delete_task)
        );

    let app = Router::new()
    .nest("/auth", auth_routes)
    .nest("/users", user_routes)
    .nest("/projects", project_routes)
    .nest("/tasks", task_routes)
    .route("/health", get(|| async {"ok"}))
    .with_state(state)
    .layer(TraceLayer::new_for_http())
    .layer(
        CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any)
    );

    let addr = env::var("SERVER_ADDR")
    .unwrap_or_else(|_| {"0.0.0.0:3000".to_string()});

    let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .expect("Failed to bind to address. Is another process using port 3000?");
    tracing::info!("Server listening on http://{}",listener.local_addr().unwrap());

    axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Server encountered a fatal error");
    


}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+c signal handler");
    };

    #[cfg(unix)]
    let sigterm = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
        .expect("Failed to install SIGTERM signal handler")
        .recv()
        .await;
    };

    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c  => {tracing::info!("Recieved Ctrl+C");}
        _ = sigterm => {tracing::info!("Recieved SIGTERM");}
    }
}
