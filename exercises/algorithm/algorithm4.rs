/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let mut ptr = self.root.as_mut();
        if ptr.is_none() {
            let new_node = Box::new(TreeNode {
                value,
                left: None,
                right: None,
            });
            self.root = Some(new_node);
            return;
        }
        while ptr.is_some() {
            let node = ptr.unwrap().as_mut();
            if node.value == value {
                return;
            }
            if node.value > value {
                if node.left.is_none() {
                    node.insert(value);
                    return;
                } else {
                    ptr = node.left.as_mut();
                }
            } else {
                if node.right.is_none() {
                    node.insert(value);
                    return;
                } else {
                    ptr = node.right.as_mut();
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut ptr = self.root.as_ref();
        if ptr.is_none() {
            return false;
        }
        while ptr.is_some() {
            let node = ptr.unwrap();
            if node.value == value {
                return true;
            }
            if node.value > value {
                ptr = node.left.as_ref();
            } else {
                ptr = node.right.as_ref();
            }
        }
        return false;
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if value > self.value {
            self.right = Some(Box::new(TreeNode::new(value)));
        } else {
            self.left = Some(Box::new(TreeNode::new(value)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}


