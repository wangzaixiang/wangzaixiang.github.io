# DataFusion HashJoin 源代码阅读

# Concepts
1. left side: build side, 完整读取，构建 hash table
2. right side: probe side: 分批读取，并进行 hash-join

# todos
-[ ] how to modify and debug 3rd party crate?
    1. update source at ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crate-name-version/src/some.rs
    2. rm -fr target/debug/.fingerprint/crate-name-hash/
    3. cargo build 会重新编译该模块。 
    4. cargo clean 不会删除 ~/.cargo/registry 下的文件

    or
    1. git clone 
    2. Cargo.toml, change as `datafusion = { path = "/Users/wangzaixiang/workspaces/github.com/datafusion/datafusion/core", version = "46.0.1"}`
    3. 修改了文件时，cargo build 会重新编译该模块，方便进行代码的调试。

- ProcessProbeBatchState
    - offset?
    - joined_probe_idx
- partition
    - 如何仅在某个 partition 进行调试？
    - 如何设定近使用1个 partition?
    - 阅读代码，理解如何从 LogicPlan 生成 PhysicalPlan
    config.optimizer.repartition_joins
    ```rust
     let config = SessionConfig::new().with_repartition_joins(false);
     let ctx = SessionContext::new_with_config(config);
    ```