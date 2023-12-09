use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreeNodeType {
  Leaf,
  Branch,
  Root,
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
        } 
        else if let Some(children) = &mut node.children {
          if let Some(found) = find_node_recursive(children, node_id) {
            return Some(found);
          }
        }
      }
      None
    }
    return find_node_recursive(&mut self.nodes, node_id);
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

    match node_id {
      Some(node_id) =>  {
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
        }
      }
      None => {
        self.nodes.push(new_leaf.clone())
      }
    }
    return new_leaf;
  }

  fn get_id(&mut self) -> usize {
    let counter = self.counter;
    println!("Counter: {}", counter);
    self.counter += 1;
    return counter;
  }
}

impl TreeNode {
  fn new_leaf(name: &str, id: usize) -> TreeNode {
    println!("Leaf id: {}", id);
    return TreeNode {
      id,
      name: name.to_string(),
      node_type: TreeNodeType::Leaf,
      children: None,
    };
  }
  fn new_branch(name: &str, id: usize, children: Option<Vec<TreeNode>>) -> TreeNode {
    println!("Branch id: {}", id);
    return TreeNode {
      id,
      name: name.to_string(),
      node_type: TreeNodeType::Branch,
      children,
    };
  }
  fn new_root(name: &str, id: usize, children: Option<Vec<TreeNode>>) -> TreeNode {
    println!("Branch id: {}", id);
    return TreeNode {
      id,
      name: name.to_string(),
      node_type: TreeNodeType::Root,
      children,
    };
  }
  fn add_child(&mut self, child: TreeNode) {
    self.children.get_or_insert_with(|| Vec::new()).push(child);
  }
}

// We test against this function if the tree is built correctly
// DONT CHANGE THE ORDER
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
  return initial_tree;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreeNodeDeserialize {
    pub id: Option<usize>,
    pub name: String,
    pub node_type: TreeNodeType,
    pub children: Option<Vec<TreeNodeDeserialize>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TreesDeserialize {
    trees: Vec<TreeNodeDeserialize>,
}
// Deserialize function
pub fn build_tree_from_json(file_content: &str) -> Tree {
  // Convert JSON to Tree
  let mut tree = Tree::new();
  if let Ok(parsed_json) = serde_json::from_str::<TreesDeserialize>(file_content) {
    println!("Parsed JSON: {:?}", parsed_json);
    for tree_node in parsed_json.trees {
        parse_json_value(&tree_node, &mut tree, None);
    }
} else {
    panic!("Failed to parse JSON");
}

println!("Initial tree: {:?}", tree);
tree
}

fn parse_json_value(
    node: &TreeNodeDeserialize,
    tree: &mut Tree,
    parent_id: Option<usize>,
) {
    let new_id = match node.id {
        Some(id) => id,
        None => tree.get_id(),
    };

    let new_node = TreeNode {
        id: new_id,
        name: node.name.clone(),
        node_type: node.node_type.clone(),
        children: None,
    };

    match parent_id {
        Some(parent_id) => {
            if let Some(parent) = tree.find_node_by_id(parent_id) {
                parent.add_child(new_node.clone());
            }
        }
        None => {
            tree.nodes.push(new_node.clone());
        }
    }

    if let Some(children) = &node.children {
        for child in children {
            parse_json_value(child, tree, Some(new_id));
        }
    }
}

pub fn build_complex_tree() -> Tree {
  // Read the content of the 'tree.json' file
  let mut file_content = String::new();
  let file_result = File::open("mock-data/tree.json");
  println!("File result: {:?}", file_result);
  if let Ok(mut file) = file_result {
      file.read_to_string(&mut file_content).ok();
  } else {
      panic!("Failed to open 'mock-data/tree.json' file");
  }
  // Convert JSON to Tree
  return build_tree_from_json(&file_content);
}