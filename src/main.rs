use axum::Router;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new();

    Ok(router.into())
}
