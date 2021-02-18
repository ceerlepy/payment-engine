# Payment Engine

Project is created by using `cargo new` 

|                |Command                          |Alternatives                                 |
|----------------|---------------------------------|---------------------------------------------|
|build           |`cargo build`                    |                                             |
|run             |`cargo run -- inputfile.csv`     |`cargo run -- inputfile.csv outputFile.csv`  |
|test            |`cargo test`                     |                                             |


**WARNIING**: **`cargo run -- transactions.csv > accounts.csv`** command extracts all error and warning println! macros to outputFile along with successful program outputs, so to seperate error and warning println! macros and successful program outputs from each other, it is highly recommended to use fallowing commands **`cargo run -- transactions.csv`** | **`cargo run -- transactions.csv accounts.csv`** to measure only successful output. when running program please consider this.


**ALL OPERATIONS ARE SUPPORTED**
- Deposit
- Withdrawal
- Dispute
- Resolve
- Chargeback

**input file format** (already added to code)
type,client,tx,amount
deposit,1,1,1.0
deposit,2,2,2.0
deposit,1,3,2.0
withdrawal,1,4,1.5
withdrawal,2,5,3.0
dispute,2,2,
chargeback,2,2,
dispute,2,1,
resolve,2,1,

**outputfile format** (already added to code)
client,available,held,total,locked
1,1.5,0.0,1.5,false
2,0.0,0.0,0.0,true
