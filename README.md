# Payment Engine

Project is created by using `cargo new` 

|                |Command                          |Alternatives                                 |
|----------------|---------------------------------|---------------------------------------------|
|build           |`cargo build`                    |                                             |
|run             |`cargo run -- inputfile.csv`     |`cargo run -- inputfile.csv outputFile.csv`  |
|test            |`cargo test`                     |                                             |


**WARNIING**: **`cargo run -- transactions.csv > accounts.csv`** command extracts all error and warning println! macros to outputFile along with successful program outputs, so to seperate error and warning println! macros and successful program outputs from each other, it is highly recommended to use fallowing commands **`cargo run -- transactions.csv`** | **`cargo run -- transactions.csv accounts.csv`** to measure only successful output. when running program please consider this.


**ALL OPERATIONS ARE SUPPORTED**__
- Deposit <br />
- Withdrawal <br />
- Dispute <br />
- Resolve <br />
- Chargeback <br />

**input file format** (already added to code) <br />
type,client,tx,amount <br />
deposit,1,1,1.0 <br />
deposit,2,2,2.0 <br />
deposit,1,3,2.0 <br />
withdrawal,1,4,1.5 <br />
withdrawal,2,5,3.0 <br />
dispute,2,2, <br />
chargeback,2,2, <br />
dispute,2,1, <br />
resolve,2,1, <br />

**outputfile format** (already added to code) <br />
client,available,held,total,locked <br />
1,1.5,0.0,1.5,false <br />
2,0.0,0.0,0.0,true <br />
