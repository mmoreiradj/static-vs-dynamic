use static_vs_dynamic::{dyn_traits, static_traits};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app_static = static_traits::router().await;
    let app_dyn = dyn_traits::router().await;

    let _ = tokio::join!(
        axum::serve(
            TcpListener::bind("127.0.0.1:3000").await.unwrap(),
            app_static.into_make_service(),
        ),
        axum::serve(
            TcpListener::bind("127.0.0.1:3001").await.unwrap(),
            app_dyn.into_make_service(),
        ),
    );
}
