#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    clippy::enum_clike_unportable_variant,
    clippy::mixed_case_hex_literals
)]

use crate::rbtree_amd::{
    rbtree_key_compare, rbtree_key_s, rbtree_key_t, rbtree_max, rbtree_min, LKP_ALL,
};

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
    // println!("rbt_red {:#?}", node);
    node.color = 1;
}

pub fn rbt_black(node: &mut rbtree_node_t) {
    // println!("rbt_black {:#?}", node);
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

#[allow(unused_assignments)]
unsafe fn hsakmt_rbtree_insert_value(
    mut temp: *mut rbtree_node_t,
    node: *mut rbtree_node_t,
    sentinel: *mut rbtree_node_t,
) {
    let mut p: *mut *mut rbtree_node_t;

    // let temp_st = &mut *(temp);
    // let node_st = &mut *(node);
    // let sentinel_st = &mut *(sentinel);

    // println!("temp_st: {:?}", temp_st);
    // println!("node_st: {:?}", node_st);
    // println!("sentinel_st: {:?}", sentinel_st);

    loop {
        let temp_st = &mut (*temp);
        let node_st = &mut (*node);

        let b = rbtree_key_compare(LKP_ALL() as u32, &node_st.key, &temp_st.key);

        // println!("root temp_st {:#?}", temp_st);

        p = if b < 0 {
            let temp_st = &mut (*temp);

            // println!("temp_st.left");
            if temp_st.left.is_null() {
                temp_st.left = sentinel;
            }
            &mut temp_st.left
        } else {
            let temp_st = &mut (*temp);

            // println!("temp_st.right");
            if temp_st.right.is_null() {
                temp_st.right = sentinel;
            }
            &mut temp_st.right
        };

        // println!("sentinel: {:?}", sentinel);

        if *p == sentinel {
            break;
        }

        // let v = *p;

        // println!("v {:#?}", v.is_null());
        // if v.is_null() {
        //     *p = sentinel;
        // }

        temp = *p;

        // break;
    }

    *p = node;

    let node_st = &mut (*node);

    node_st.parent = temp;
    node_st.left = sentinel;
    node_st.right = sentinel;

    rbt_red(node_st);
}

pub unsafe fn hsakmt_rbtree_insert(tree: &mut rbtree_s, mut node: *mut rbtree_node_s) {
    /* a binary tree insert */
    let root_st = &mut (*tree.root);

    let sentinel = &mut tree.sentinel as *mut rbtree_node_t;
    let node_st = &mut (*node);

    if root_st.key == tree.sentinel.key {
        node_st.parent = std::ptr::null_mut();
        node_st.left = sentinel;
        node_st.right = sentinel;
        rbt_black(node_st);

        let root = &mut tree.root as *mut *mut rbtree_node_t;

        *root = node;

        // println!("first node");
        // println!("node_st: {:#?}", node_st);

        // println!("first node inserted {:#?}", &(**root));

        return;
    }

    // println!("TODO hsakmt_rbtree_insert_value ");

    let root = &mut tree.root as *mut *mut rbtree_node_t;

    // println!("sentinel: {:#?}", tree.sentinel);

    hsakmt_rbtree_insert_value(*root, node, sentinel);

    // println!("node inserted {:#?}", &(**root));

    /* re-balance tree */

    while node != *root && rbt_is_red(&(*node_st.parent)) {
        let node_parent = &mut (*node_st.parent);
        let node_parent_parent = &mut (*node_parent.parent);

        if node_st.parent == node_parent_parent.left {
            // let temp = node_parent_parent.right;
            let temp_st = &mut (*node_parent_parent.right);

            if rbt_is_red(temp_st) {
                rbt_black(node_parent);
                rbt_black(temp_st);
                rbt_red(node_parent_parent);

                node = node_parent_parent;
            } else {
                if node == node_parent.right {
                    node = node_st.parent;

                    rbtree_left_rotate(root, sentinel, node_st);
                }

                rbt_black(node_parent);
                rbt_red(node_parent_parent);

                rbtree_right_rotate(root, sentinel, node_parent_parent);
            }
        } else {
            let node_parent = &mut (*node_st.parent);
            let node_parent_parent = &mut (*node_parent.parent);

            // let temp = node_parent_parent.left;
            let temp_st = &mut (*node_parent_parent.left);

            if rbt_is_red(temp_st) {
                rbt_black(node_parent);
                rbt_black(temp_st);
                rbt_red(node_parent_parent);

                node = node_parent_parent;
            } else {
                if node == node_parent.left {
                    node = node_st.parent;

                    rbtree_right_rotate(root, sentinel, node_st);
                }

                rbt_black(node_parent);
                rbt_red(node_parent_parent);

                rbtree_left_rotate(root, sentinel, node_parent_parent);
            }
        }

        println!("while node {:#?}", node);
    }

    let root_st = &mut *(tree.root);

    rbt_black(root_st);
}

#[allow(unused_assignments)]
pub unsafe fn hsakmt_rbtree_delete(tree: &mut rbtree_s, node: *mut rbtree_node_s) {
    // // let root_st = &mut *(tree.root);
    //
    // let sentinel = &mut tree.sentinel;
    // let root = &mut tree.root as *mut *mut rbtree_node_t;
    //
    // let node_st = &mut *(node);
    //
    // let mut temp: *mut rbtree_node_t = std::ptr::null_mut();
    // let mut subst: *mut rbtree_node_t = std::ptr::null_mut();
    //
    // /* a binary tree delete */
    //
    // if node_st.left == sentinel {
    //     temp = node_st.right;
    //     subst = node;
    // } else if node_st.right == sentinel {
    //     temp = node_st.left;
    //     subst = node;
    // } else {
    //     let subst = rbtree_min(node_st.right, sentinel);
    //
    //     let subst_st = &mut (*subst);
    //
    //     if subst_st.left != sentinel {
    //         temp = subst_st.left;
    //     } else {
    //         temp = subst_st.right;
    //     }
    // }
    //
    // if subst == *root {
    //     *root = temp;
    //     rbt_black(&mut *temp);
    //
    //     return;
    // }
    //
    // let subst_st = &mut (*subst);
    // let subst_parent = &mut (*subst_st.parent);
    //
    // let temp_st = &mut (*temp);
    //
    // let red = rbt_is_red(subst_st);
    //
    // if subst == subst_parent.left {
    //     subst_parent.left = temp;
    // } else {
    //     subst_parent.right = temp;
    // }
    //
    // if subst == node {
    //     temp_st.parent = subst_st.parent;
    // } else {
    //     if subst_st.parent == node {
    //         temp_st.parent = subst;
    //     } else {
    //         temp_st.parent = subst_st.parent;
    //     }
    //
    //     subst_st.left = node_st.left;
    //     subst_st.right = node_st.right;
    //     subst_st.parent = node_st.parent;
    //     rbt_copy_color(subst_st, node_st);
    //
    //     let node_parent = &mut *(node_st.parent);
    //
    //     if node == *root {
    //         *root = subst;
    //     } else {
    //         if node == node_parent.left {
    //             node_parent.left = subst;
    //         } else {
    //             node_parent.right = subst;
    //         }
    //     }
    //
    //     let subst_left = &mut *(subst_st.left);
    //     let subst_right = &mut *(subst_st.right);
    //
    //     if subst_st.left != sentinel {
    //         subst_left.parent = subst;
    //     }
    //
    //     if subst_st.right != sentinel {
    //         subst_right.parent = subst;
    //     }
    // }
    //
    // if red {
    //     return;
    // }
    //
    // /* a delete fixup */
    //
    // while temp != *root && rbt_is_black(temp_st) {
    //     let temp_parent = &mut *(temp_st.parent);
    //
    //     if temp_st == &(*temp_parent.left) {
    //         let mut w = temp_parent.right;
    //         let w_st = &mut (*w);
    //
    //         if rbt_is_red(w_st) {
    //             rbt_black(w_st);
    //             rbt_red(&mut *temp_st.parent);
    //             rbtree_left_rotate(root, sentinel, &mut *temp_st.parent);
    //             w = temp_parent.right;
    //         }
    //
    //         let w_left = &mut (*w_st.left);
    //         let w_right = &mut (*w_st.right);
    //
    //         if rbt_is_black(w_left) && rbt_is_black(w_right) {
    //             rbt_red(w_st);
    //             temp = temp_st.parent;
    //         } else {
    //             if rbt_is_black(w_right) {
    //                 rbt_black(w_left);
    //                 rbt_red(w_st);
    //
    //                 rbtree_right_rotate(root, sentinel, w_st);
    //                 w = temp_parent.right;
    //             }
    //
    //             rbt_copy_color(w_st, temp_parent);
    //             rbt_black(temp_parent);
    //             rbt_black(w_right);
    //
    //             rbtree_left_rotate(root, sentinel, temp_parent);
    //             temp = *root;
    //         }
    //     } else {
    //         let mut w = temp_parent.left;
    //         let w_st = &mut (*w);
    //
    //         if rbt_is_red(w_st) {
    //             rbt_black(w_st);
    //             rbt_red(temp_parent);
    //             rbtree_right_rotate(root, sentinel, temp_parent);
    //             w = temp_parent.left;
    //         }
    //
    //         let w_left = &mut (*w_st.left);
    //         let w_right = &mut (*w_st.right);
    //
    //         if rbt_is_black(w_left) && rbt_is_black(w_right) {
    //             rbt_red(w_st);
    //             temp = temp_parent;
    //         } else {
    //             if rbt_is_black(w_left) {
    //                 rbt_black(w_right);
    //                 rbt_red(w_st);
    //                 rbtree_left_rotate(root, sentinel, w_st);
    //                 w = temp_parent.left;
    //             }
    //
    //             rbt_copy_color(w_st, temp_parent);
    //             rbt_black(temp_parent);
    //             rbt_black(w_left);
    //             rbtree_right_rotate(root, sentinel, temp_parent);
    //             temp = *root;
    //         }
    //     }
    // }
    //
    // rbt_black(temp_st);
}

pub unsafe fn rbtree_left_rotate(
    root: *mut *mut rbtree_node_t,
    sentinel: *mut rbtree_node_t,
    node: &mut rbtree_node_t,
) {
    let temp = node.right;
    let temp_st = &mut (*node.right);

    node.right = temp_st.left;

    let temp_left = &mut (*temp_st.left);

    if temp_st.left.is_null() {
        temp_st.left = sentinel;
    }

    if temp_st.left != sentinel {
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

pub unsafe fn rbtree_right_rotate(
    root: *mut *mut rbtree_node_t,
    sentinel: *mut rbtree_node_t,
    node: &mut rbtree_node_t,
) {
    let temp = node.left;
    let temp_st = &mut *(node.left);

    node.left = temp_st.right;

    let temp_right = &mut (*temp_st.right);

    if temp_st.right.is_null() {
        temp_st.right = sentinel;
    }

    if temp_st.right != sentinel {
        temp_right.parent = node;
    }

    temp_st.parent = node.parent;

    let node_parent = &mut (*node.parent);

    if node == &(**root) {
        *root = temp;
    } else if node == &mut (*node_parent.right) {
        node_parent.right = temp;
    } else {
        node_parent.left = temp;
    }

    temp_st.right = node;
    node.parent = temp;
}

pub unsafe fn hsakmt_rbtree_prev(
    tree: &mut rbtree_t,
    mut node: *mut rbtree_node_t,
) -> *mut rbtree_node_t {
    // rbtree_node_t  *root, *sentinel, *parent;

    let sentinel = &mut tree.sentinel;
    let node_st = &mut (*node);

    if node_st.left != sentinel {
        return rbtree_max(node_st.left, sentinel);
    }

    let root = tree.root;

    loop {
        let parent = node_st.parent;

        if node == root {
            return std::ptr::null_mut();
        }

        let parent_st = &mut (*parent);

        if node == parent_st.right {
            return parent;
        }

        node = parent
    }
}

pub unsafe fn print_tree(tree: &rbtree_s) {
    println!("tree: {:#?}", tree);
    let root = &(*tree.root);
    println!("tree.root: {:#?}", root);

    let root_left = &(*root.left);
    println!("tree.root.left: {:#?}", root_left);

    // let root_right = &(*root.right);
    // println!("tree.root.right: {:#?}", root_right);
    //
    // let root_left_left = &(*root_left.left);
    // println!("tree.root.left.left: {:#?}", root_left_left);
    //
    // let root_left_right = &(*root_left.right);
    // println!("tree.root.left.right: {:#?}", root_left_right);
    //
    // let root_right_left = &(*root_right.left);
    // println!("tree.root.left.left: {:#?}", root_right_left);
    //
    // let root_right_right = &(*root_right.right);
    // println!("tree.root.left.right: {:#?}", root_right_right);
}

#[cfg(test)]
mod tests_rbtree {
    use super::*;
    use crate::rbtree_amd::{rbtree_lookup_nearest, rbtree_min_max, LEFT, RIGHT};

    #[test]
    fn test_insert() {
        let mut tree = rbtree_s {
            root: std::ptr::null_mut(),
            sentinel: Default::default(),
        };

        rbtree_init(&mut tree);

        let mut node_1 = rbtree_node_s::default();
        node_1.key = rbtree_key_s {
            addr: 1,
            size: 4096,
        };

        let mut node_2 = rbtree_node_s::default();
        node_2.key = rbtree_key_s {
            addr: 2,
            size: 4096,
        };

        let mut node_3 = rbtree_node_s::default();
        node_3.key = rbtree_key_s {
            addr: 3,
            size: 4096,
        };

        let mut node_4 = rbtree_node_s::default();
        node_4.key = rbtree_key_s {
            addr: 4,
            size: 4096,
        };

        let mut node_5 = rbtree_node_s::default();
        node_5.key = rbtree_key_s {
            addr: 5,
            size: 4096,
        };

        unsafe {
            hsakmt_rbtree_insert(&mut tree, &mut node_1);
            hsakmt_rbtree_insert(&mut tree, &mut node_2);
            hsakmt_rbtree_insert(&mut tree, &mut node_3);
            hsakmt_rbtree_insert(&mut tree, &mut node_4);

            hsakmt_rbtree_delete(&mut tree, &mut node_2);

            // rbtree_min(&mut node_3, &mut tree.sentinel);
            //
            // hsakmt_rbtree_prev(&mut tree, &mut node_4);
            //
            // let _rn = rbtree_min_max(&mut tree, RIGHT as i32);
            // let _rn = rbtree_min_max(&mut tree, LEFT as i32);

            // let _rn = rbtree_lookup_nearest(&mut tree, &node_3.key, LKP_ALL() as u32, RIGHT as i32);

            // let _rn = rbtree_lookup_nearest(&mut tree, &node_1.key, LKP_ALL() as u32, LEFT as i32);
            // let _rn = rbtree_lookup_nearest(&mut tree, &node_2.key, LKP_ALL() as u32, LEFT as i32);
            // let _rn = rbtree_lookup_nearest(&mut tree, &node_3.key, LKP_ALL() as u32, LEFT as i32);
            // let _rn = rbtree_lookup_nearest(&mut tree, &node_4.key, LKP_ALL() as u32, LEFT as i32);

            // let _rn = rbtree_lookup_nearest(&mut tree, &node_5.key, LKP_ALL() as u32, LEFT as i32);

            println!("tree: {:#?}", tree);
            let root = &(*tree.root);
            println!("tree.root: {:#?}", root);

            let root_left = &(*root.left);
            println!("tree.root.left: {:#?}", root_left);

            let root_right = &(*root.right);
            println!("tree.root.right: {:#?}", root_right);

            let root_left_left = &(*root_left.left);
            println!("tree.root.left.left: {:#?}", root_left_left);

            let root_left_right = &(*root_left.right);
            println!("tree.root.left.right: {:#?}", root_left_right);

            let root_right_left = &(*root_right.left);
            println!("tree.root.left.left: {:#?}", root_right_left);

            let root_right_right = &(*root_right.right);
            println!("tree.root.left.right: {:#?}", root_right_right);
        }
    }
}
