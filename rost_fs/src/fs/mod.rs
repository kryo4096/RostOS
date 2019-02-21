use crate::disk::block::Block;
use crate::disk::{Disk, DiskAddress};
use crate::node::*;

use rand::prelude::*;
use rand::rngs::SmallRng;
use spin::{Once, RwLock};

use alloc::prelude::*;
use alloc::string::String;

pub mod path;

use alloc::vec::Vec;

use core::ops::Deref;

pub use crate::node::NodeID;

pub fn random_id() -> i64 {
    static RNG: Once<RwLock<SmallRng>> = Once::new();
    RNG.call_once(|| RwLock::new(SmallRng::from_seed([12; 16])))
        .write()
        .next_u64() as _
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeHeader {
    pub node_type: u64,
}

impl NodeHeader {
    pub const DIRECTORY: NodeHeader = NodeHeader { node_type: 1 };
    pub const FILE: NodeHeader = NodeHeader { node_type: 0 };
}

pub const NODEHDR_SIZE: usize = core::mem::size_of::<NodeHeader>();

#[derive(Debug)]
pub enum FSError {
    InvalidID,
    InvalidHdr,
    InvalidName(String),
    NotAFile,
    NotADirectory,
    ChildMissing(String),
    MalformedChild(String),
}

pub type FSResult<T> = Result<T, FSError>;

pub fn get_header(tree: &mut NodeTree<impl Disk>, id: NodeID) -> FSResult<NodeHeader> {
    let node = tree.search_node(id).map_err(|_| FSError::InvalidID)?;

    let mut buf = Vec::new();

    node.read_data_slice(&mut buf, 0, NODEHDR_SIZE as u64);

    if buf.len() == 0 {
        return Err(FSError::InvalidHdr);
    }

    Ok(unsafe { (*(buf.as_ptr() as *const NodeHeader)).clone() })
}

pub fn get_content(tree: &mut NodeTree<impl Disk>, id: NodeID, buf: &mut Vec<u8>) -> FSResult<()> {
    let node = tree.search_node(id).map_err(|_| FSError::InvalidHdr)?;
    node.read_data_slice(
        buf,
        NODEHDR_SIZE as u64,
        node.data_size().map_err(|_| FSError::InvalidID)?,
    );
    Ok(())
}

pub fn create_node(tree: &mut NodeTree<impl Disk>) -> NodeID {
    let mut id = 0;

    while tree.insert_node(id).is_err() {
        id = random_id();
    }

    id
}

pub fn write(id: NodeID, hdr: NodeHeader, data: &[u8], tree: &mut NodeTree<impl Disk>) {
    let mut buf = Vec::new();
    let hdr: [u8; NODEHDR_SIZE] = unsafe { *(&hdr as *const _ as *const _) };

    buf.extend(hdr.iter());

    buf.extend(data.iter());

    let node = tree.search_node(id).expect("invalid node used");

    node.write_data(&buf);
}

pub fn is_file(tree: &mut NodeTree<impl Disk>, id: NodeID) -> FSResult<()> {
    if get_header(tree, id)?.node_type != NodeHeader::FILE.node_type {
        Err(FSError::NotAFile)
    } else {
        Ok(())
    }
}

pub fn is_dir(tree: &mut NodeTree<impl Disk>, id: NodeID) -> FSResult<()> {
    if get_header(tree, id)?.node_type != NodeHeader::DIRECTORY.node_type {
        Err(FSError::NotADirectory)
    } else {
        Ok(())
    }
}

pub fn validate_name(name: &[u8]) -> FSResult<&[u8]> {
    if !name.contains(&b';') && !name.contains(&b'\n') && name.len() > 0 {
        Ok(name)
    } else {
        Err(FSError::InvalidName(
            String::from_utf8_lossy(name).to_string(),
        ))
    }
}

pub fn write_file(tree: &mut NodeTree<impl Disk>, id: NodeID, data: &[u8]) -> FSResult<()> {
    is_file(tree, id)?;

    write(id, NodeHeader::FILE, data, tree);

    Ok(())
}

pub fn read_file(tree: &mut NodeTree<impl Disk>, id: NodeID, vec: &mut Vec<u8>) -> FSResult<()> {
    is_file(tree, id)?;

    get_content(tree, id, vec);

    Ok(())
}

pub fn add_child(
    tree: &mut NodeTree<impl Disk>,
    dir: NodeID,
    name: &[u8],
    node: NodeID,
) -> FSResult<()> {
    let name = validate_name(name)?;

    is_dir(tree, dir)?;

    let mut buf = Vec::new();

    get_content(tree, dir, &mut buf)?;

    buf.extend(name.iter());
    buf.push(b';');
    buf.extend(unsafe { core::mem::transmute::<NodeID, [u8; 8]>(node).iter() });
    buf.push(b'\n');

    write(dir, NodeHeader::DIRECTORY, &buf, tree);

    Ok(())
}

pub fn list_children(
    tree: &mut NodeTree<impl Disk>,
    dir: NodeID,
) -> FSResult<Vec<(Vec<u8>, NodeID)>> {
    let mut buf = Vec::new();
    get_content(tree, dir, &mut buf)?;

    let mut children = vec![];

    for line in buf.split(|&b| b == b'\n') {
        if line.len() < 1 {
            continue;
        }

        let (name, id) = {
            let mut s = line.split(|&b| b == b';');

            (
                s.next().ok_or(FSError::MalformedChild(
                    String::from_utf8_lossy(line).into(),
                ))?,
                s.next()
                    .filter(|id| id.len() == 8)
                    .ok_or(FSError::MalformedChild(
                        String::from_utf8_lossy(line).into(),
                    ))?,
            )
        };

        let id = unsafe { *(id.as_ptr() as *const NodeID) };

        children.push((name.into(), id))
    }

    Ok(children)
}

pub fn get_child(tree: &mut NodeTree<impl Disk>, dir: NodeID, name: &[u8]) -> FSResult<NodeID> {
    let name = validate_name(name)?;

    let mut buf = Vec::new();
    get_content(tree, dir, &mut buf)?;

    for (child_name, id) in list_children(tree, dir)? {
        if child_name == name {
            return Ok(id);
        }
    }

    Err(FSError::ChildMissing(
        String::from_utf8_lossy(name).to_string(),
    ))
}

pub fn create(tree: &mut NodeTree<impl Disk>, path: &[u8], from: NodeID) -> FSResult<NodeID> {
    let tail = path::tail(path);

    let dir = open(tree, tail, from)?;

    let new_node = create_node(tree);

    add_child(tree, dir, path::head(path), new_node);

    Ok(new_node)
}

pub fn open(tree: &mut NodeTree<impl Disk>, path: &[u8], from: NodeID) -> FSResult<NodeID> {
    let mut dir = from;

    if path == b"" || path == b"/" {
        return Ok(from);
    }

    for segment in path::iter(path) {
        dir = get_child(tree, dir, segment)?;
    }

    Ok(dir)
}
