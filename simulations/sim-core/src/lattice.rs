/// 2D lattice index helpers with periodic boundary conditions.

/// Convert (x, y) to flat index.
#[inline]
pub fn idx(x: u32, y: u32, width: u32) -> usize {
    (y * width + x) as usize
}

/// Wrap coordinate with periodic boundaries.
#[inline]
pub fn wrap(val: i32, size: u32) -> u32 {
    ((val % size as i32) + size as i32) as u32 % size
}

/// The four cardinal directions: Right, Down, Left, Up.
pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/// Total number of sites in the lattice.
#[inline]
pub fn num_sites(width: u32, height: u32) -> usize {
    (width * height) as usize
}
