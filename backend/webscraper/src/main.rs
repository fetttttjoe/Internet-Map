use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::cell::Cell;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod test_tree;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TreeNodeType {
    Leaf,
    Branch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    id: usize,
    name: String,
    node_type: TreeNodeType,
    children: Option<Vec<TreeNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tree {
    nodes: Vec<TreeNode>,
    counter: Cell<usize>,
}

impl Tree {
    fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            counter: Cell::new(1),
        }
    }
    // New method to find a leaf node by ID
    fn find_node_by_id(&mut self, node_id: usize) -> Option<&mut TreeNode> {
        // Recursively search for the leaf node with the specified ID
        fn find_node_recursive(nodes: &mut Vec<TreeNode>, node_id: usize) -> Option<&mut TreeNode> {
            for node in nodes.iter_mut() {
                if node.id == node_id {
                    return Some(node);
                } else if let Some(children) = &mut node.children {
                    if let Some(found) = find_node_recursive(children, node_id) {
                        return Some(found);
                    }
                }
            }
            None
        }

        find_node_recursive(&mut self.nodes, node_id)
    }
    fn add_node(
        &mut self,
        name: &str,
        node_id: Option<usize>,
        append_to_node: bool,
        update: bool,
    ) -> TreeNode {
        println!("New leaf: {} with id {:?}", name, node_id);

        if update {
            if let Some(node_id) = node_id {
                // Check if the leaf with the specified ID already exists
                if let Some(existing_leaf) = self.find_node_by_id(node_id) {
                    // Update the existing leaf with the new name
                    existing_leaf.name = name.to_string();
                    return existing_leaf.clone(); // No need to clone, just return the existing leaf
                }
            }
        }

        // The leaf doesn't exist, create a new one
        let new_id = self.get_id();
        let new_leaf = TreeNode::new_leaf(name, new_id);

        if let Some(node_id) = node_id {
            if let Some(node) = self.find_node_by_id(node_id) {
                if append_to_node {
                    // Append new leaf to the existing parent
                    node.add_child(new_leaf.clone());
                } else {
                    // Create a new parent only if we are not appending to an existing leaf
                    let new_parent =
                        TreeNode::new_branch(name, new_id, Some(vec![new_leaf.clone()]));
                    if let Some(existing_node) = self.find_node_by_id(node_id) {
                        // Append new parent to the existing node
                        existing_node.add_child(new_parent.clone());
                    } else {
                        // Node with the specified id not found, add as a new node
                        self.nodes.push(new_parent);
                    }
                }
            } else {
                self.nodes.push(new_leaf.clone());
            }
        } else {
            // If no node_id is specified, treat it as a root node
            let new_leaf = TreeNode::new_leaf(name, new_id);
            self.nodes.push(new_leaf.clone());
        }

        new_leaf
    }

    fn get_id(&self) -> usize {
        let counter = self.counter.get();
        println!("Counter: {}", counter);
        self.counter.set(counter + 1);
        counter
    }
}

impl TreeNode {
    fn new_leaf(name: &str, id: usize) -> TreeNode {
        println!("Leaf id: {}", id);

        TreeNode {
            id,
            name: name.to_string(),
            node_type: TreeNodeType::Leaf,
            children: None,
        }
    }

    fn new_branch(name: &str, id: usize, children: Option<Vec<TreeNode>>) -> TreeNode {
        println!("Branch id: {}", id);

        TreeNode {
            id,
            name: name.to_string(),
            node_type: TreeNodeType::Branch,
            children: children,
        }
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.get_or_insert_with(|| Vec::new()).push(child);
    }
}

#[get("/tree")]
fn get_tree(node: &rocket::State<Arc<Mutex<Tree>>>) -> Json<Tree> {
    let locked_tree = node.lock().expect("Failed to lock tree for reading");
    Json(locked_tree.clone())
}

#[post("/add-node", data = "<new_node>")]
fn add_node(node: &rocket::State<Arc<Mutex<Tree>>>, new_node: Json<TreeNode>) -> Json<Tree> {
    let mut locked_tree = node.lock().expect("Failed to lock tree for modification");
    let node_to_insert = new_node.into_inner();

    // Assuming the node_id is specified in the JSON payload
    let node_id = node_to_insert
        .children
        .as_ref()
        .and_then(|children| children.get(0).map(|child| child.id));

    locked_tree.add_node(&node_to_insert.name, node_id, true, false);
    Json(locked_tree.clone())
}

#[rocket::main]
async fn main() {
    // Create the Tree here and add some leaves and branches
    let mut initial_tree = build_initial_tree();

    let arc_tree = Arc::new(Mutex::new(initial_tree));
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let rocket = rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![get_tree, add_node])
        .manage(arc_tree);
    rocket.launch().await;
}

pub fn build_initial_tree() -> Tree {
    let mut initial_tree = Tree::new();

    // Adding root node
    let root_node = initial_tree.add_node("Root", None, false, false);
    // Adding Leaf1
    let leaf1 = initial_tree.add_node("Leaf1", Some(root_node.id), true, false);

    // Adding Leaf2
    let leaf2 = initial_tree.add_node("Leaf2", Some(root_node.id), true, false);

    // Adding Leaf3 with children
    let leaf3 = initial_tree.add_node("Leaf3", Some(root_node.id), true, false);

    let leaf7 = initial_tree.add_node("Leaf7", Some(leaf3.id), true, false);
    let leaf13 = initial_tree.add_node("Leaf13", Some(leaf7.id), true, false);
    let leaf14 = initial_tree.add_node("Leaf14", Some(leaf7.id), true, false);

    let leaf8 = initial_tree.add_node("Leaf8", Some(leaf3.id), true, false);
    let leaf9 = initial_tree.add_node("Leaf9", Some(leaf8.id), true, false);

    // Adding Leaf4
    let leaf4 = initial_tree.add_node("Leaf4", Some(root_node.id), true, false);

    // Adding Leaf5
    let leaf5 = initial_tree.add_node("Leaf5", Some(root_node.id), true, false);

    initial_tree
}
