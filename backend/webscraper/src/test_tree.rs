// tree_test.rs
use super::*;

#[test]
fn test_initial_tree_structure() {
    let initial_tree = build_initial_tree();

    // Define the expected tree structure
    let expected_structure = TreeNode {
        id: 1,
        name: "Root".to_string(),
        node_type: TreeNodeType::Branch,
        children: Some(vec![
            TreeNode {
                id: 2,
                name: "Leaf1".to_string(),
                children: None,
                node_type: TreeNodeType::Leaf,
            },
            TreeNode {
                id: 3,
                name: "Leaf2".to_string(),
                children: None,
                node_type: TreeNodeType::Leaf,
            },
            TreeNode {
                id: 4,
                name: "Leaf3".to_string(),
                node_type: TreeNodeType::Branch,
                children: Some(vec![
                    TreeNode {
                        id: 5,
                        name: "Leaf7".to_string(),
                        node_type: TreeNodeType::Branch,
                        children: Some(vec![
                            TreeNode {
                                id: 6,
                                name: "Leaf13".to_string(),
                                children: None,
                                node_type: TreeNodeType::Leaf,
                            },
                            TreeNode {
                                id: 7,
                                name: "Leaf14".to_string(),
                                children: None,
                                node_type: TreeNodeType::Leaf,
                            },
                        ]),
                    },
                    TreeNode {
                        id: 8,
                        name: "Leaf8".to_string(),
                        node_type: TreeNodeType::Branch,
                        children: Some(vec![TreeNode {
                            id: 9,
                            name: "Leaf9".to_string(),
                            children: None,
                            node_type: TreeNodeType::Leaf,
                        }]),
                    },
                ]),
            },
            TreeNode {
                id: 10,
                name: "Leaf4".to_string(),
                children: None,
                node_type: TreeNodeType::Leaf,
            },
            TreeNode {
                id: 11,
                name: "Leaf5".to_string(),
                children: None,
                node_type: TreeNodeType::Leaf,
            },
        ]),
    };

    // Perform the comparison
    assert_eq!(
        compare_tree_structure(&initial_tree.nodes, &expected_structure),
        true
    );
}
fn compare_tree_structure(actual: &Vec<TreeNode>, expected: &TreeNode) -> bool {
    // Helper function to recursively compare TreeNodes
    fn compare_TreeNode(actual: &TreeNode, expected: &TreeNode) -> bool {
        actual.id == expected.id
            && actual.name == expected.name
            && compare_children(&actual.children, &expected.children)
    }

    // Helper function to compare child vectors
    fn compare_children(actual: &Option<Vec<TreeNode>>, expected: &Option<Vec<TreeNode>>) -> bool {
        match (actual, expected) {
            (Some(actual_children), Some(expected_children)) => {
                if actual_children.len() != expected_children.len() {
                    return false;
                }
                for (actual_child, expected_child) in actual_children.iter().zip(expected_children)
                {
                    if !compare_TreeNode(actual_child, expected_child) {
                        return false;
                    }
                }
                true
            }
            (None, None) => true,
            _ => false,
        }
    }

    // Iterate over the TreeNodes and compare with the expected structure
    actual
        .iter()
        .zip(std::iter::once(expected))
        .all(|(actual_TreeNode, expected_TreeNode)| {
            compare_TreeNode(actual_TreeNode, expected_TreeNode)
        })
}
