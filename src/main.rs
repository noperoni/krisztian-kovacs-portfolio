
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::http::{header, HeaderName, HeaderValue, StatusCode};
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use portfolio::app::*;
    use portfolio::db;
    use std::net::SocketAddr;
    use std::time::Duration;
    use tower::{timeout::TimeoutLayer, ServiceBuilder};
    use tower_http::set_header::SetResponseHeaderLayer;
    use tracing::{info, warn};

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize structured logging (respects RUST_LOG env var)
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("portfolio=info".parse().unwrap())
                .add_directive("tower_http=info".parse().unwrap()),
        )
        .init();

    // Initialize database pool
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    // Run migrations
    db::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    // Cleanup stale rate limit records (older than 24 hours)
    match db::cleanup_rate_limits(&pool).await {
        Ok(deleted) if deleted > 0 => info!(deleted, "Cleaned up stale rate limit records"),
        Ok(_) => {}
        Err(e) => warn!(?e, "Failed to cleanup rate limits"),
    }

    info!("Database connected and migrations applied");

    // Create shared HTTP client for external API calls (GitHub)
    let http_client = reqwest::Client::builder()
        .user_agent("krisztian-kovacs-portfolio/1.0")
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Clone resources for the context closure
    let pool_for_context = pool.clone();
    let http_client_for_context = http_client.clone();

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                // Provide database pool as context for server functions
                provide_context(pool_for_context.clone());
                // Provide shared HTTP client for external API calls
                provide_context(http_client_for_context.clone());
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        // Request timeout middleware (30 seconds)
        .layer(
            ServiceBuilder::new()
                .layer(axum::error_handling::HandleErrorLayer::new(
                    |_: axum::BoxError| async { StatusCode::REQUEST_TIMEOUT },
                ))
                .layer(TimeoutLayer::new(Duration::from_secs(30))),
        )
        // Security headers middleware
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ));
        // Note: CSP header with nonce is set dynamically in CspHeader component

    // run our app with hyper
    info!(%addr, "Server starting");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
