#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    clippy::enum_clike_unportable_variant,
    clippy::mixed_case_hex_literals
)]

use crate::rbtree_amd::{rbtree_key_compare, rbtree_key_s, rbtree_key_t, LKP_ALL};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct rbtree_node_s {
    pub key: rbtree_key_t,
    pub left: *mut rbtree_node_t,
    pub right: *mut rbtree_node_t,
    pub parent: *mut rbtree_node_t,
    pub color: u8,
    pub data: u8,
}

impl Default for rbtree_node_s {
    fn default() -> Self {
        Self {
            key: rbtree_key_s { addr: 0, size: 0 },
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent: std::ptr::null_mut(),
            color: 0,
            data: 0,
        }
    }
}

pub type rbtree_node_t = rbtree_node_s;

#[derive(Debug, PartialEq, Eq)]
pub struct rbtree_s {
    pub root: *mut rbtree_node_t,
    pub sentinel: rbtree_node_t,
}

pub type rbtree_t<'a> = rbtree_s;

// #define rbt_red(node)			((node)->color = 1)
// #define rbt_black(node)			((node)->color = 0)
// #define rbt_is_red(node)		((node)->color)
// #define rbt_is_black(node)		(!rbt_is_red(node))
// #define rbt_copy_color(n1, n2)		(n1->color = n2->color)
//
// /* a sentinel must be black */
//
// #define rbtree_sentinel_init(node)	rbt_black(node)}

// #define rbtree_init(tree)				\
// rbtree_sentinel_init(&(tree)->sentinel);	\
// (tree)->root = &(tree)->sentinel;

pub fn rbt_red(node: &mut rbtree_node_t) {
    node.color = 1;
}

pub fn rbt_black(node: &mut rbtree_node_t) {
    node.color = 0;
}

pub fn rbt_is_red(node: &rbtree_node_t) -> bool {
    node.color == 1
}

pub fn rbt_is_black(node: &rbtree_node_t) -> bool {
    !rbt_is_red(node)
}

pub fn rbt_copy_color(n1: &mut rbtree_node_t, n2: &rbtree_node_t) {
    n1.color = n2.color;
}

pub fn rbtree_sentinel_init(node: &mut rbtree_node_t) {
    rbt_black(node);
}

pub fn rbtree_init(tree: &mut rbtree_t) {
    rbtree_sentinel_init(&mut tree.sentinel);
    tree.root = &mut tree.sentinel as *mut rbtree_node_t;
}

unsafe fn hsakmt_rbtree_insert_value(
    mut temp: &mut rbtree_node_t,
    node: *mut rbtree_node_t,
    sentinel: &mut rbtree_node_t,
) {
    let mut p: *mut *mut rbtree_node_t = std::ptr::null_mut();
    let node_st = &mut *(node);

    loop {
        let b = rbtree_key_compare(LKP_ALL() as u32, &node_st.key, &temp.key);

        p = if b < 0 {
            &mut temp.left
        } else {
            &mut temp.right
        };

        if &(**p) == sentinel {
            break;
        }

        temp = &mut **p;
    }

    *p = node;

    node_st.parent = temp;
    node_st.left = sentinel;
    node_st.right = sentinel;

    rbt_red(node_st);
}

pub unsafe fn hsakmt_rbtree_insert(tree: &mut rbtree_s, mut node: *mut rbtree_node_s) {
    /* a binary tree insert */
    let root_st = &mut *(tree.root);

    let sentinel = &mut tree.sentinel as *mut rbtree_node_t;
    let root = &mut tree.root as *mut *mut rbtree_node_t;

    let node_st = &mut *(node);

    if root_st.key.eq(&tree.sentinel.key) {
        node_st.parent = std::ptr::null_mut();
        node_st.left = sentinel;
        node_st.right = sentinel;
        rbt_black(node_st);

        *root = node;

        return;
    }

    hsakmt_rbtree_insert_value(root_st, node, &mut tree.sentinel);

    /* re-balance tree */

    while node != root_st && rbt_is_red(&*(node_st.parent)) {
        let node_parent = &mut *(node_st.parent);
        let node_parent_parent = &mut *(node_parent.parent);

        if node_st.parent == node_parent_parent.left {
            // let temp = node_parent_parent.right;
            let temp_st = &mut *(node_parent_parent.right);

            if rbt_is_red(temp_st) {
                rbt_black(node_parent);
                rbt_black(temp_st);
                rbt_red(node_parent_parent);

                node = node_parent_parent;
            } else {
                if (node as *mut rbtree_node_t) == node_parent.right {
                    node = node_parent

                    // rbtree_left_rotate(root, sentinel, node);
                }

                rbt_black(node_parent);
                rbt_red(node_parent_parent);

                // rbtree_right_rotate(root, sentinel, node_parent_parent);
            }
        } else {
            let node_parent = &mut *(node_st.parent);
            let node_parent_parent = &mut *(node_parent.parent);

            // let temp = node_parent_parent.left;
            let temp_st = &mut *(node_parent_parent.left);

            if rbt_is_red(temp_st) {
                rbt_black(node_parent);
                rbt_black(temp_st);
                rbt_red(node_parent_parent);

                node = node_parent_parent;
            } else {
                if node == node_parent.left {
                    node = node_parent;

                    // rbtree_right_rotate(root, sentinel, node);
                }

                rbt_black(node_parent);
                rbt_red(node_parent_parent);

                // rbtree_left_rotate(root, sentinel, node->parent->parent);
            }
        }
    }

    let root_st = &mut *(tree.root);

    rbt_black(root_st);
}

pub unsafe fn rbtree_left_rotate(
    root: *mut *mut rbtree_node_t,
    sentinel: &rbtree_node_t,
    node: &mut rbtree_node_t,
) {
    let temp = node.right;
    let temp_st = &mut *(node.right);

    node.right = temp_st.left;

    let temp_left = &mut (*temp_st.left);

    if temp_left != sentinel {
        temp_left.parent = node;
    }

    temp_st.parent = node.parent;

    let node_parent = &mut (*node.parent);

    if node == &(**root) {
        *root = temp;
    } else if node == &mut (*node_parent.left) {
        node_parent.left = temp;
    } else {
        node_parent.right = temp;
    }

    temp_st.left = node;
    node.parent = temp;
}
