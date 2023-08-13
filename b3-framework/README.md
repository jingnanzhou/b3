# Blockchain Building Blocks (B3) Frameworks

B3 includes four network frameworks: 
* Solo Chain Proof of Concept (POC)  
* Solo Chain Minimum Viable Product (MVP) 
* Relay Chain Minimum Viable Product (MVP) 
* Para Chain Minimum Viable Product (MVP) 

Each network framework has the followinh file structure

* node
    node directory contains basic node services including main function
* net-defs
  contains directory contains list of blockchain network definitions.

* net-selection
  net-selection choose one chain in net-defs directory
  the sample code to choose chain in Cargo.toml

  ```
[features]

<mark> default = ["rococo"] </mark>

westend = ["westend-chain-runtime", "westend-chain-spec", "westend-chain-runtime-constants" ]

rococo = ["rococo-chain-runtime", "rococo-chain-spec", "rococo-chain-runtime-constants" ]

``` 
