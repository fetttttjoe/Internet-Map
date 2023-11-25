use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use webscraper::{
    ip_crawler::{handle_request, scan_ips},
    tree::{build_initial_tree, Tree, TreeNode},
};

async fn get_tree(state: Extension<Arc<Mutex<Tree>>>) -> Json<Tree> {
    let locked_tree = state.lock().expect("Failed to lock tree for reading");
    println!("Locked tree: {:?}", locked_tree);
    Json(locked_tree.clone())
}

pub async fn add_node(state: Extension<Arc<Mutex<Tree>>>, new_node: Json<TreeNode>) -> Json<Tree> {
    let mut locked_tree = state.lock().expect("Failed to lock tree for modification");
    let node_to_insert = new_node.0;
    let node_id = node_to_insert
        .children
        .as_ref()
        .and_then(|children| children.get(0).map(|child| child.id));
    locked_tree.add_node(&node_to_insert.name, node_id, true, false);
    Json(locked_tree.clone())
}
fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods(vec![axum::http::Method::GET, axum::http::Method::POST])
        .allow_origin(Any);
    let router = Router::new()
        .route("/tree", get(get_tree))
        .route("/add-node", post(add_node))
        .layer(cors);
    router
}
#[tokio::main]
async fn main() {
    // Create the Tree here and add some leaves and branches
    let initial_tree = build_initial_tree();
    let arc_tree = Arc::new(Mutex::new(initial_tree));
    tracing_subscriber::fmt::init();

    let router = create_router().layer(Extension(arc_tree));
    let start_ip = "128.65.209.28";
    let end_ip = "128.65.209.40";
    let port = 80;
    let response = scan_ips(start_ip, end_ip, port).await;
    for res in response {
        println!("Response: {:?}", res);
    }
    let addr = SocketAddr::from(([127, 0, 0, 1], 3200));
    println!("Listening on http://{} ", addr);
    tracing::debug!("listening on {} ", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
