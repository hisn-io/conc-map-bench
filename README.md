# conc-map-bench

conc-map-bench uses the bustle benchmarking harness. This is a port of the well regarded libcuckoo benchmark.

## Workloads

The benchmark measures performance under varying load conditions. This is done
because a map suitable for one workload may not be suitable for another.

### Read Heavy

A read heavy model with few inserts, removals and updates. Models caching of data in places such as webservers and disk page caches.
```
read   98%
insert  1%
remove  1%
update  0%
```

### Exchange

Insert and remove heavy model that replicates a scenario where the map is used to exchange data.
```
read    10%
insert  40%
remove  40%
update  10%
```

### Rapid Grow

An insert heavy model that replicates load in a scenario where the map is used to gather large amounts of data under a short burst.
```
read    5%
insert 80%
remove  5%
update 10%
```

## How to run it?

```sh
mv results results.bk
./scripts/bench.sh
./scripts/plot.sh
```

## Results

Machine: Apple M1 Pro (2021 14-inch MacBook Pro)

OS: macOS 14.5

See the `results/` directory.

### Read Heavy (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/ReadHeavy.std.throughput.svg) | ![](results/ReadHeavy.std.latency.svg)

### Exchange (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/Exchange.std.throughput.svg) | ![](results/Exchange.std.latency.svg)

### Rapid Grow (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/RapidGrow.std.throughput.svg) | ![](results/RapidGrow.std.latency.svg)

### Read Heavy (ahash)
| | |
:-------------------------:|:-------------------------:
![](results/ReadHeavy.ahash.throughput.svg) | ![](results/ReadHeavy.ahash.latency.svg)

### Exchange (ahash)
| | |
:-------------------------:|:-------------------------:
![](results/Exchange.ahash.throughput.svg) | ![](results/Exchange.ahash.latency.svg)

### Rapid Grow (ahash)
| | |
:-------------------------:|:-------------------------:
![](results/RapidGrow.ahash.throughput.svg) | ![](results/RapidGrow.ahash.latency.svg)

## Rapid Grow (fxhash)
| | |
:-------------------------:|:-------------------------:
![](results/RapidGrow.fxhash.throughput.svg) | ![](results/RapidGrow.fxhash.latency.svg)

### Read Heavy (fxhash)
| | |
:-------------------------:|:-------------------------:
![](results/ReadHeavy.fxhash.throughput.svg) | ![](results/ReadHeavy.fxhash.latency.svg)

### Exchange (fxhash)
| | |
:-------------------------:|:-------------------------:
![](results/Exchange.fxhash.throughput.svg) | ![](results/Exchange.fxhash.latency.svg)

## Rapid Grow (foldhash)
| | |
:-------------------------:|:-------------------------:
![](results/RapidGrow.foldhash.throughput.svg) | ![](results/RapidGrow.foldhash.latency.svg)

### Read Heavy (foldhash)
| | |
:-------------------------:|:-------------------------:
![](results/ReadHeavy.foldhash.throughput.svg) | ![](results/ReadHeavy.foldhash.latency.svg)

### Exchange (foldhash)
| | |
:-------------------------:|:-------------------------:
![](results/Exchange.foldhash.throughput.svg) | ![](results/Exchange.foldhash.latency.svg)
