#![allow(
    non_camel_case_types,
    non_snake_case,
    dead_code,
    non_upper_case_globals,
    clippy::enum_clike_unportable_variant,
    clippy::mixed_case_hex_literals
)]

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
    pub size: u64,
}

pub type rbtree_key_t = rbtree_key_s;

pub fn rbtree_key(addr: u64, size: u64) -> rbtree_key_t {
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
