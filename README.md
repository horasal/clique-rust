Rust implementation of clique/pseudo-clique enumeration algorithms.

### Clique

* BronKeybosch

* Tomita

![Time](https://cdn.rawgit.com/zhaihj/clique-rust/master/benchtime/graph-tomita.svg "Tomita Algorithm")


### How to use

```
git clone https://github.com/zhaihj/clique-rust

cd clique-rust && cargo build

./target/debug/clique-rust ./your-graph-file-in-edge-list tomita
```
