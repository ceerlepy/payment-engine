# Payment Engine

Project is created by using `cargo new` 

|                |Command                          |Alternatives                                 |
|----------------|---------------------------------|---------------------------------------------|
|build           |`cargo build`                    |                                             |
|run             |`cargo run -- inputfile.csv`     |`cargo run -- inputfile.csv outputFile.csv`  |
|test            |`cargo test`                     |                                             |


**WARNIING**: **`cargo run -- transactions.csv > accounts.csv`** command extracts all error and warning println! macros to outputFile along with successful program outputs, so to seperate error and warning println! macros and successful program outputs from each other, it is highly recommended to use fallowing commands **`cargo run -- transactions.csv`** | **`cargo run -- transactions.csv accounts.csv`** to measure only successful output. when running program please consider this.
