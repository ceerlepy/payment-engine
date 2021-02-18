# Payment Engine

Project is created by using `cargo new` 

|                |Command                          |Alternatives                                 |
|----------------|---------------------------------|---------------------------------------------|
|build           |`cargo build`                    |                                             |
|run             |`cargo run -- inputfile.csv`     |`cargo run -- inputfile.csv outputFile.csv`  |
|test            |`cargo test`                     |                                             |


**WARNIING**: **`cargo run -- transactions.csv > accounts.csv`** command extracts all error and warning println! macros to outputFile along with successful program outputs, so to seperate error and warning println! macros and successful program outputs from each other, it is highly recommended to use fallowing commands **`cargo run -- transactions.csv`** | **`cargo run -- transactions.csv accounts.csv`** to measure only successful output. when running program please consider this.


**ALL OPERATIONS ARE SUPPORTED**__
- Deposit__
- Withdrawal__
- Dispute__
- Resolve__
- Chargeback__

**input file format** (already added to code)__
type,client,tx,amount__
deposit,1,1,1.0__
deposit,2,2,2.0__
deposit,1,3,2.0__
withdrawal,1,4,1.5__
withdrawal,2,5,3.0__
dispute,2,2,__
chargeback,2,2,__
dispute,2,1,__
resolve,2,1,__

**outputfile format** (already added to code)__
client,available,held,total,locked__
1,1.5,0.0,1.5,false__
2,0.0,0.0,0.0,true__
