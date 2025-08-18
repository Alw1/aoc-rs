#2. Module hierarchy
Rust's module system lets you organize code within the single crate. Here's how you can structure it: 

src/
├── lib.rs
├── main.rs
├── inputs/           <-- Store input files here
│   ├── 2023/
│   │   ├── 1.txt
│   │   ├── 2.txt
│   │   └── ...
│   ├── 2024/
│   │   ├── 1.txt
│   │   ├── 2.txt
│   │   └── ...
│   └── ...
├── years/            <-- Contains modules for each year
│   ├── mod.rs
│   ├── y2023/
│   │   ├── mod.rs
│   │   ├── day_01.rs
│   │   ├── day_02.rs
│   │   └── ...
│   ├── y2024/
│   │   ├── mod.rs
│   │   ├── day_01.rs
│   │   ├── day_02.rs
│   │   └── ...
│   └── ...
└── utils/            <-- Shared utilities (parsing, algorithms, etc.)
    ├── mod.rs
    ├── parsers.rs
    ├── algorithms.rs
    └── aoc_rust
My Advent of Code solutions written in Rust 
