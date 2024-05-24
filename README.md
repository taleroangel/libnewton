# 🪐 libnewton
A library for interacting with Prism 🌈
> Visit https://github.com/taleroangel/prism for more information about the **Prism Project**

This repository contains most of the implementation of the _[Prism](https://github.com/taleroangel/prism)_ protocol with all the numeric constants defined inside _protobuf_ files in order to make porting to many programming languages easier

## ✒️ List of supported programming languages

Current implemented languages and stages:

| Language | Status |
| -------- | ------ |
| Protobuf | 🟢      |
| Rust     | 🟢      |

### ✴️ Base (protobuf)
The base implementtion of libnewton (constants names and values like the _InstructionSet_, _Registers_ and _AddressingMode_) are defined in a set of protobuf files found in the [protobuf](./protobuf/) directory, this is the base from which new libraries should be built upon and can be a direct dependency

### 🦀 Rust
The rust library depends on the **[protoc-gen-prost](https://crates.io/crates/protoc-gen-prost)** crate for generating the protobuf base files rust implementation. These files are commited to source control but can be recompiled using the following command invoked from parent directory:

    protoc -I ./protobuf --prost_out=rust/src/proto ./protobuf/*.proto


## 🔭 Newton
Newton is the name given to the _Prism Instruction Interpreter_ therefore a _Newton Interpreter_ is required in every slave device. Instructions are interpreted in _Prism Binary Format_ which can be assembled from a _Prism Assembly Language_ using this library