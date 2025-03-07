pub use self::{
    btreemap::ParkingLotRwLockBTreeMapTable, btreemap::StdRwLockBTreeMapTable,
    chashmap::CHashMapTable, contrie::ContrieTable, crossbeam_skiplist::CrossbeamSkipMapTable,
    dashmap::DashMapTable, evmap::EvmapTable, papaya::PapayaTable, papaya::PinnedPapayaTable,
    scc::SccMapTable, std::ParkingLotRwLockStdHashMapTable, std::StdRwLockStdHashMapTable,
};

mod btreemap;
mod chashmap;
mod contrie;
mod crossbeam_skiplist;
mod dashmap;
mod evmap;
//mod flurry;
mod papaya;
mod scc;
mod std;

type Value = u32;
