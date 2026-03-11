use alloy_primitives::address;
use axum::{Router, routing::get};
use axum::response::IntoResponse;
use x402_axum::X402Middleware;
use x402_chain_eip155::{KnownNetworkEip155, V1Eip155Exact};
use x402_types::networks::USDC;
use tokio::net::TcpListener;
use http::StatusCode;

async fn my_handler() -> impl IntoResponse {
    (StatusCode::OK, "This is VIP content!")
}

#[tokio::main]
async fn main() {
    let x402 = X402Middleware::new("https://facilitator.x402.rs");

    let app: Router = Router::new().route(
        "/protected",
        get(my_handler).layer(
            x402.with_price_tag(V1Eip155Exact::price_tag(
                address!("0x0d2Dc4E9ebc1465E86Fdf6ab18377CB82eCf7548"),
                USDC::base_sepolia().parse("0.01").unwrap(),
            ))
        ),
    );
    let app = app.into_make_service();
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}