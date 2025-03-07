use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::iter;
use std::{fmt::Debug, io};

use bustle::*;
use structopt::StructOpt;

use crate::{adapters::*, record::Record, workloads};

#[derive(Debug)]
pub enum HasherKind {
    Std,
    FoldHash,
}

fn parse_hasher_kind(hasher: &str) -> Result<HasherKind, &str> {
    match hasher {
        "std" => Ok(HasherKind::Std),
        "foldhash" => Ok(HasherKind::FoldHash),
        _ => Err("invalid hasher, must be one of 'std' or 'foldhash'"),
    }
}

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short, long)]
    pub workload: workloads::WorkloadKind,
    #[structopt(short, long, default_value = "1")]
    pub operations: f64,
    #[structopt(long)]
    pub threads: Option<Vec<u32>>,
    #[structopt(short, long, parse(try_from_str = parse_hasher_kind))]
    pub hasher: HasherKind,
    #[structopt(long)]
    pub skip: Option<Vec<String>>, // TODO: use just `Vec<String>`.
    #[structopt(long)]
    pub csv: bool,
    #[structopt(long)]
    pub csv_no_headers: bool,
    #[structopt(long)]
    pub max_threads: Option<usize>,
}

type Handler = Box<dyn FnMut(&str, u32, &Measurement)>;

fn case<C>(name: &str, options: &Options, handler: &mut Handler)
where
    C: Collection,
    <C::Handle as CollectionHandle>::Key: Send + Debug,
{
    if options
        .skip
        .as_ref()
        .and_then(|s| s.iter().find(|s| s == &name))
        .is_some()
    {
        println!("-- {} [skipped]", name);
        return;
    } else {
        println!("-- {}", name);
    }

    let gen_threads = || {
        let mut n = num_cpus::get();
        if let Some(max) = options.max_threads {
            n = max;
        }

        match n {
            0..=10 => (1..=n as u32).collect(),
            11..=16 => iter::once(1)
                .chain((0..=n as u32).step_by(2).skip(1))
                .collect(),
            _ => iter::once(1)
                .chain((0..=n as u32).step_by(4).skip(1))
                .collect(),
        }
    };

    let threads = options
        .threads
        .as_ref()
        .cloned()
        .unwrap_or_else(gen_threads);

    for n in &threads {
        let m = workloads::create(options, *n).run_silently::<C>();
        handler(name, *n, &m);
    }
    println!();
}

fn run(options: &Options, h: &mut Handler) {
    //case::<StdRwLockBTreeMapTable<u64>>("std:sync::RwLock<BTreeMap>", options, h);
    case::<ParkingLotRwLockBTreeMapTable<u64>>("parking_lot::RwLock<BTreeMap>", options, h);
    //case::<CHashMapTable<u64>>("CHashMap", options, h);
    case::<CrossbeamSkipMapTable<u64>>("CrossbeamSkipMap", options, h);

    match options.hasher {
        HasherKind::Std => run_hasher_variant::<RandomState>(options, h),
        HasherKind::FoldHash => run_hasher_variant::<foldhash::fast::RandomState>(options, h),
    }
}

fn run_hasher_variant<H>(options: &Options, h: &mut Handler)
where
    H: Default + Clone + Send + Sync + BuildHasher + 'static,
{
    //case::<StdRwLockStdHashMapTable<u64, H>>("std::sync::RwLock<StdHashMap>", options, h);
    case::<ParkingLotRwLockStdHashMapTable<u64, H>>("parking_lot::RwLock<StdHashMap>", options, h);
    case::<DashMapTable<u64, H>>("DashMap 7.0.0-rc2", options, h);
    case::<PapayaTable<u64, H>>("Papaya", options, h);
    case::<PinnedPapayaTable<u64, H>>("Papaya refresh-every-8", options, h);
    case::<FlurryTable<u64, H>>("Flurry", options, h);
    case::<EvmapTable<u64, H>>("Evmap", options, h);
    case::<ContrieTable<u64, H>>("Contrie", options, h);
    case::<SccMapTable<u64, H>>("SccMap", options, h);
}

pub fn bench(options: &Options) {
    println!("== {:?}", options.workload);

    let mut handler = if options.csv {
        let mut wr = csv::WriterBuilder::new()
            .has_headers(!options.csv_no_headers)
            .from_writer(io::stderr());

        Box::new(move |name: &str, n, m: &Measurement| {
            wr.serialize(Record {
                name: name.into(),
                total_ops: m.total_ops,
                threads: n,
                spent: m.spent,
                throughput: m.throughput,
                latency: m.latency,
            })
            .expect("cannot serialize");
            wr.flush().expect("cannot flush");
        }) as Handler
    } else {
        Box::new(|_: &str, n, m: &Measurement| {
            eprintln!(
                "total_ops={}\tthreads={}\tspent={:.1?}\tlatency={:?}\tthroughput={:.0}op/s",
                m.total_ops, n, m.spent, m.latency, m.throughput,
            );
        }) as Handler
    };

    run(options, &mut handler);
}
