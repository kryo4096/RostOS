
use core::cell::UnsafeCell;

use crate::disk::block::{self, Block};
use crate::disk::{self, Disk, DiskAddress};
use crate::node::node::{self, Node};
use crate::node::{tree, NodeSet};
use alloc::vec::Vec;
#[derive(Default)]
struct DebugDisk {
    blocks: UnsafeCell<Vec<Block>>,
    size: u64,
}

impl DebugDisk {
    pub fn new(size: u64) -> Self {
        Self {
            blocks: UnsafeCell::new(vec![[0; 4096]; size as usize]),
            size
        }
    }
}

impl Disk for DebugDisk {
    fn get_block(&self, index: DiskAddress) -> Option<&mut Block> {
        unsafe { (*self.blocks.get()).get_mut(index.index()? as usize) }
    }

    fn block_count(&self) -> u64 {
        self.size
    }
}

impl core::fmt::Debug for DebugDisk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "DebugDisk ({} KB)", self.size * 4)
    }
}

#[test]
fn test_magic() {
    let mut disk = DebugDisk::new(1024);

    let name = b"TEST PARTITION";

    disk::format(&disk, name);

    assert_eq!(block::get_root_block(&disk).magic, block::MAGIC);
}

#[test]
fn block_alloc_dealloc() {
    let mut disk = DebugDisk::new(1024);

    disk::format(&disk, b"TEST");

    for i in 1..100 {
        assert_eq!(DiskAddress::block(i), block::allocate_block(&disk).unwrap());
    }

    for i in 1..100 {
        block::deallocate_block(&disk, DiskAddress::block(i)).expect("Deallocation failed.");
    }

    for i in 1..100 {
        assert_eq!(
            DiskAddress::block(100 - i),
            block::allocate_block(&disk).unwrap()
        );
    }

    assert_eq!(
        DiskAddress::block(100),
        block::allocate_block(&disk).unwrap()
    );
}

#[test]
fn node_alloc_dealloc() {
    let mut disk = DebugDisk::new(1024);

    disk::format(&mut disk, b"TEST");

    let addr = node::allocate_node(&mut disk).expect("allocation #1 failed");

    assert_eq!(
        node::get_node(&mut disk, addr)
            .expect("NULL POINTER EXCEPTION")
            .key,
        Node::EMPTY.key
    );

    node::deallocate_node(&mut disk, addr).expect("deallocation failed");

    assert_eq!(
        node::allocate_node(&mut disk).expect("allocation #2 failed"),
        addr
    );
}

#[test]
fn tree_test() {
    let mut disk = DebugDisk::new(1024);
    disk::format(&mut disk, b"TEST");

    let mut addr = [(DiskAddress::NULL, 0); 1024];

    for i in 1..addr.len() {
        addr[i].1 = if i % 2 == 0 { i as i64 } else { -(i as i64) };
        addr[i].0 =
            tree::insert_node(&disk, addr[i].1).expect(&format!("node insertion failed at {}", i));
    }
}


#[test]
fn public_api_test() {
    let mut disk = DebugDisk::new(1024);

    let fs = crate::node::NodeTree::new(&mut disk);

    let node = fs.insert_node(100).expect("node insertion failed");

    let same_node = fs.search_node(100).expect("node search failed");

    assert_eq!(node, same_node, "found node not valid");

    let data = include_bytes!("test.bin"); 

    node.write_data(data).expect("writing failed");

    let mut buf = vec!();

    node.read_data(&mut buf).expect("reading failed");

    assert_eq!(data.len(), buf.len());

    for index in 0 .. buf.len() {
        assert_eq!(data[index], buf[index], "data corrupted at {}", index);
    }

}

