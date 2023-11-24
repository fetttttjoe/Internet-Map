use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreeNodeType {
    Leaf,
    Branch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    pub id: usize,
    pub name: String,
    pub node_type: TreeNodeType,
    pub children: Option<Vec<TreeNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    nodes: Vec<TreeNode>,
    counter: usize,
}

impl Tree {
    fn new() -> Self {
        Tree {
            nodes: Vec::new(),
            counter: 1,
        }
    }
    pub fn get_nodes(&self) -> &Vec<TreeNode> {
        &self.nodes
    }
    fn find_node_by_id(&mut self, node_id: usize) -> Option<&mut TreeNode> {
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

    pub fn add_node(
        &mut self,
        name: &str,
        node_id: Option<usize>,
        append_to_node: bool,
        update: bool,
    ) -> TreeNode {
        println!("New leaf: {} with id {:?}", name, node_id);

        if update {
            if let Some(node_id) = node_id {
                if let Some(existing_leaf) = self.find_node_by_id(node_id) {
                    existing_leaf.name = name.to_string();
                    return existing_leaf.clone();
                }
            }
        }

        let new_id = self.get_id();
        let new_leaf = TreeNode::new_leaf(name, new_id);

        if let Some(node_id) = node_id {
            if let Some(node) = self.find_node_by_id(node_id) {
                if append_to_node {
                    node.add_child(new_leaf.clone());
                } else {
                    let new_parent =
                        TreeNode::new_branch(name, new_id, Some(vec![new_leaf.clone()]));
                    if let Some(existing_node) = self.find_node_by_id(node_id) {
                        existing_node.add_child(new_parent.clone());
                    } else {
                        self.nodes.push(new_parent);
                    }
                }
            } else {
                self.nodes.push(new_leaf.clone());
            }
        } else {
            let new_leaf = TreeNode::new_leaf(name, new_id);
            self.nodes.push(new_leaf.clone());
        }

        new_leaf
    }

    fn get_id(&mut self) -> usize {
        let counter = self.counter;
        println!("Counter: {}", counter);
        self.counter += 1;
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
            children,
        }
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.get_or_insert_with(|| Vec::new()).push(child);
    }
}

pub fn build_initial_tree() -> Tree {
    let mut initial_tree = Tree::new();

    // Adding root node
    let root_node = initial_tree.add_node("Root", None, false, false);
    // Adding Leaf1
    let _leaf1 = initial_tree.add_node("Leaf1", Some(root_node.id), true, false);

    // Adding Leaf2
    let _leaf2 = initial_tree.add_node("Leaf2", Some(root_node.id), true, false);

    // Adding Leaf3 with children
    let leaf3 = initial_tree.add_node("Leaf3", Some(root_node.id), true, false);

    let leaf7 = initial_tree.add_node("Leaf7", Some(leaf3.id), true, false);
    let _leaf13 = initial_tree.add_node("Leaf13", Some(leaf7.id), true, false);
    let _leaf14 = initial_tree.add_node("Leaf14", Some(leaf7.id), true, false);

    let leaf8 = initial_tree.add_node("Leaf8", Some(leaf3.id), true, false);
    let _leaf9 = initial_tree.add_node("Leaf9", Some(leaf8.id), true, false);

    // Adding Leaf4
    let _leaf4 = initial_tree.add_node("Leaf4", Some(root_node.id), true, false);

    // Adding Leaf5
    let _leaf5 = initial_tree.add_node("Leaf5", Some(root_node.id), true, false);

    initial_tree
}
