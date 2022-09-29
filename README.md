# chess2vec
Tools for computing latent representation of a chess position.  
Extract hard coded features from a given move history:

- Material
- Expansion Factor
- Mobility
- Package Density
- Pawn Structure
- ...


## wasm-bindgen

- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- ```~/.cargo/bin/wasm-pack build --target no-modules```  
(Outputs JS that can be natively imported as an ES module in a browser, but the WebAssembly must be manually instantiated and loaded.
The JS is included on a page and modifies global state, and doesn't support as many wasm-bindgen features as web)

## native

``cargo run --example test``