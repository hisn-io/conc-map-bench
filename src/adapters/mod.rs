#![allow(unused_imports)]

pub use self::{
    contrie::ContrieTable, dashmap::DashMapTable, evmap::EvmapTable, flurry::FlurryTable,
    papaya::PapayaTable, scc::SccIndexTable, scc::SccMapTable,
    std::ParkingLotRwLockStdHashMapTable, std::StdRwLockStdHashMapTable,
};

mod contrie;
mod dashmap;
mod evmap;
mod flurry;
mod papaya;
mod scc;
mod std;

type Value = u32;
