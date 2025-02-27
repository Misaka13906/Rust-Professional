/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;
use std::clone::Clone;

#[derive(Debug)]
#[derive(Clone)]
struct TreeNode<T>
where
    T: Ord + Clone,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord + Clone,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord + Clone,
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
    T: Ord + Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn dfs_insert(now: &mut Box<TreeNode<T>>, value: &T) {
        if *value < now.value {
            if now.left.is_some() {
                Self::dfs_insert(now.left.as_mut().unwrap(), value);
            } else {
                now.left = Some(Box::new(TreeNode::new(value.clone())));
            }
        }
        if *value > now.value {
            if now.right.is_some() {
                Self::dfs_insert(now.right.as_mut().unwrap(), value);
            } else {
                now.right = Some(Box::new(TreeNode::new(value.clone())));
            }
        }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value.clone())));
            return;
        }
        Self::dfs_insert(self.root.as_mut().unwrap(), &value);
    }

    fn dfs(now: TreeNode<T>, value: &T) -> bool {
        if *value == now.value {
            return true;
        }
        if *value < now.value && now.left.is_some() {
            return Self::dfs(*now.left.unwrap(), value);
        }
        if *value > now.value && now.right.is_some() {
            return Self::dfs(*now.right.unwrap(), value);
        }
        return false;
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        if self.root.is_none() {
            return false;
        }
        return Self::dfs(*self.root.clone().unwrap(), &value);
    }
}

impl<T> TreeNode<T>
where
    T: Ord + Clone,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        // unused
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


