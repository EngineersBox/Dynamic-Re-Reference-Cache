use memmap::{MmapMut, MmapOptions};

pub struct MmapRegion {
    mmap: MmapMut,
    seek_marker: u64,
}

impl MmapRegion {

    pub fn new(mmap: MmapMut) -> Self {
        MmapRegion {
            mmap,
            seek_marker: 0
        }
    }

    pub fn

}