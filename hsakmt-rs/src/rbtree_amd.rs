#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    clippy::enum_clike_unportable_variant,
    clippy::mixed_case_hex_literals
)]

use crate::rbtree::{rbtree_min, rbtree_node_t, rbtree_t};

pub const LEFT: usize = 0;
pub const RIGHT: usize = 1;
pub const MID: usize = 2;

// typedef struct rbtree_key_s rbtree_key_t;
// struct rbtree_key_s {
//     #define ADDR_BIT 0
//     #define SIZE_BIT 1
//     unsigned long addr;
//     unsigned long size;
// };
// #define BIT(x) (1<<(x))
// #define LKP_ALL (BIT(ADDR_BIT) | BIT(SIZE_BIT))
// #define LKP_ADDR (BIT(ADDR_BIT))
// #define LKP_ADDR_SIZE (BIT(ADDR_BIT) | BIT(SIZE_BIT))

const ADDR_BIT: usize = 0;
const SIZE_BIT: usize = 1;

pub fn BIT(x: u64) -> u64 {
    1 << (x)
}

// pub fn LKP_ALL() -> u64 {
//     BIT(ADDR_BIT as u64) | BIT(SIZE_BIT as u64)
// }

pub fn LKP_ALL() -> u64 {
    (1 << (0)) | (1 << (1))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct rbtree_key_s {
    pub addr: u64,
    pub size: i64,
}

pub type rbtree_key_t = rbtree_key_s;

pub fn rbtree_key(addr: u64, size: i64) -> rbtree_key_t {
    rbtree_key_t { addr, size }
}

/*
 * compare addr, size one by one
 */
pub fn rbtree_key_compare<'a>(type_v: u32, key1: &'a rbtree_key_t, key2: &'a rbtree_key_t) -> i32 {
    let b_1 = type_v & 1 << ADDR_BIT;
    let b_2 = key1.addr != key2.addr;

    if b_1 > 0 && b_2 {
        return if key1.addr > key2.addr { 1 } else { -1 };
    }

    let b_3 = type_v & 1 << SIZE_BIT;
    let b_4 = key1.size != key2.size;

    if b_3 > 0 && b_4 {
        return if key1.size > key2.size { 1 } else { -1 };
    }

    0
}

pub unsafe fn rbtree_max(
    mut node: *mut rbtree_node_t,
    sentinel: *mut rbtree_node_t,
) -> *mut rbtree_node_t {
    let node_st = &mut (*node);

    while node_st.right != sentinel {
        node = node_st.right;
    }

    node
}

pub unsafe fn rbtree_min_max(tree: &mut rbtree_t, lr: i32) -> *mut rbtree_node_t {
    let sentinel = &mut tree.sentinel;
    let mut node = tree.root;

    if node == sentinel {
        return std::ptr::null_mut();
    }

    if lr == LEFT as i32 {
        node = rbtree_min(node, sentinel);
    } else if lr == RIGHT as i32 {
        node = rbtree_max(node, sentinel);
    }

    node
}

pub unsafe fn rbtree_lookup_nearest(
    rbtree: &mut rbtree_t,
    key: &rbtree_key_t,
    type_v: u32,
    lr: i32,
) -> *mut rbtree_node_t {
    let mut n: *mut rbtree_node_t = std::ptr::null_mut();

    let mut node = rbtree.root;
    let sentinel = &mut rbtree.sentinel;

    while node != sentinel {
        let node_st = &(*node);
        let rc = rbtree_key_compare(type_v, key, &node_st.key);

        if rc < 0 {
            if lr == RIGHT as i32 {
                n = node;
            }

            node = node_st.left;

            continue;
        }

        if rc > 0 {
            if lr == LEFT as i32 {
                n = node;
            }

            node = node_st.right;

            continue;
        }

        return node;
    }

    n
}
