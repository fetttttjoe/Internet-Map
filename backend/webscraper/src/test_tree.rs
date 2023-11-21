// tree_test.rs
use super::*;

#[derive(Debug, PartialEq)]
struct Node<'a> {
    id: usize,
    name: &'a str,
    children: Option<Vec<Node<'a>>>,
}

#[test]
fn test_initial_tree_structure() {
    let initial_tree = build_initial_tree();

    // Define the expected tree structure
    let expected_structure = Node {
        id: 2,
        name: "Root",
        children: Some(vec![
            Node { id: 3, name: "Leaf1", children: None },
            Node { id: 4, name: "Leaf2", children: None },
            Node {
                id: 5,
                name: "Leaf3",
                children: Some(vec![
                    Node {
                        id: 6,
                        name: "Leaf7",
                        children: Some(vec![
                            Node { id: 7, name: "Leaf13", children: None },
                            Node { id: 8, name: "Leaf14", children: None },
                        ]),
                    },
                    Node {
                        id: 9,
                        name: "Leaf8",
                        children: Some(vec![Node { id: 10, name: "Leaf9", children: None }]),
                    },
                ]),
            },
            Node { id: 11, name: "Leaf4", children: None },
            Node { id: 12, name: "Leaf5", children: None },
        ]),
    };

    // Perform the comparison
    assert_eq!(compare_tree_structure(&initial_tree.nodes, &expected_structure), true);
}

fn compare_tree_structure(actual_nodes: &[TreeNode], expected_node: &Node) -> bool {
    println!("Comparing Actual Nodes {:?}", actual_nodes);
    println!("Comparing Expected Nodes {:?}", expected_node);

    if actual_nodes.len() != expected_node.children.as_deref().unwrap_or_default().len() {
        return false;
    }

    for (actual, expected) in actual_nodes.iter().zip(expected_node.children.as_deref().unwrap_or_default()) {
        if actual.name != expected.name || !compare_tree_structure(actual.children.as_deref().unwrap_or_default(), expected) {
            return false;
        }
    }

    true
}
