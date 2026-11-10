# generalized-map

> A Rust implementation of **Generalized Maps (G-maps)** for combinatorial topology, supporting topological operations, mesh manipulation, and geometric modeling.

![Rust](https://img.shields.io/badge/language-Rust-orange)
![Status](https://img.shields.io/badge/status-active-green)

---

## 📌 Overview

**generalized-map** is a Rust crate for representing and manipulating **generalized maps**, a data structure used in computational geometry and topology.

It provides tools for:

- Representing n-dimensional cellular subdivisions
- Performing topological operations (sew, unsew, contract, expand)
- Traversing orbits and cells
- Managing attributes on elements
- Building and modifying combinatorial structures

This makes it suitable for:

- Mesh processing
- CAD / geometry kernels
- Topological modeling
- Research in combinatorial maps

---

## Generalized map foundations and demonstrations (disclaimer)

The current project has been inspired by the work of the team of researchers at CGAL who seriously and in detail discuss and demonstrate varyous advanced topics of computational geometry. Generalized maps are a step further from the traditional triangulated meshes, half-edges, etc in its abstraction. To learn more about generalized maps, please go to <https://doc.cgal.org/latest/Generalized_map/>.
   
The current Rust project mostly followed the book by Guillaume Damiand and Pascal Lienhardt (G. Damiand and P. Lienhardt, Combinatorial Maps: Efficient Data Structures for Computer Graphics and Image Processing. Boca Raton, FL, USA: CRC Press, 2014.) in everything concerning the algorithms particularities.
   
This Rust project is NOT a FFI port of the CGAL library (it is written in pure Rust from scratch) and might not include all the functionality of a large mature project developed by a team of professional scientists. Therefore, you are welcome to collaborate on this project and help the author add any missing functionality you might wish to see here.


---

## ✨ Features

- 🧩 Generalized map (G-map) data structure
- 🔗 Dart-based topology representation
- 🔄 Topological operations:
  - Sew / unsew
  - Contract / expand
  - Insert / remove
  - Extrude / triangulate
- 🌐 Orbit traversal utilities
- 🏷️ Attribute system for embedding data
- 📊 Inspection and statistics tools
- 📁 Serialization utilities
- 📈 Visualization support (Python + JSON)

---

## 🛠️ Tech Stack

- Language: **Rust**
- Serialization: `serde`
- Data handling: custom structures
- Visualization: Python scripts (matplotlib)

---

## 📂 Project Structure

```
gmap/
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── dart.rs
│   ├── attribute.rs
│   ├── index.rs
│   ├── index_allocator.rs
│   ├── iterators.rs
│   ├── orbits.rs
│   ├── inspect.rs
│   ├── output.rs
│   ├── gmap/
│   │   ├── mod.rs
│   │   ├── darts.rs
│   │   ├── links.rs
│   │   ├── sew.rs
│   │   ├── remove.rs
│   │   ├── insert.rs
│   │   ├── expand.rs
│   │   ├── contract.rs
│   │   ├── extrude.rs
│   │   ├── triangulate.rs
│   │   ├── shapes.rs
│   │   ├── shapes_embedded.rs
│   │   ├── attributes.rs
│   │   ├── marks.rs
│   │   ├── stats.rs
│   │   ├── close_boundary.rs
│   │   ├── chamfer.rs
│
├── visual/
│   ├── plot.py
│   ├── map.json
│   ├── nodes_and_edges.dat
```

---

## 🚀 Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/evnekdev/gmap.git
cd gmap
```

### 2. Build

```bash
cargo build
```

### 3. Run example

```bash
cargo run
```

---

## 📊 Usage

Example (conceptual):

```rust
use gmap::GMap;

fn main() {
    let mut map = GMap::new();

    // Create darts / structure
    let d1 = map.add_dart();
    let d2 = map.add_dart();

    // Link darts (topological relation)
    map.sew(0, d1, d2);

    // Traverse orbit
    for d in map.orbit(0, d1) {
        println!("Visited dart {:?}", d);
    }
}
```

---

## 🧩 Core Concepts

For the verified alpha-slot, cell-orbit, incidence and external-adapter contract,
see [Topology contract and verification](docs/topology-contract.md).

### Dart

- Fundamental unit of a G-map
- Represents a directed atomic element

---

### Involutions (α-functions)

- Define connectivity between darts
- Encode topology

---

### Orbits

- Traversal of connected components
- Used to identify cells (vertices, edges, faces, etc.)

---

### Attributes

- Attach data to cells
- Managed via attribute system

---

### Topological Operations

- **Sew / Unsew** — connect or disconnect cells
- **Insert / Remove** — modify structure
- **Expand / Contract** — refine or simplify
- **Extrude / Triangulate** — geometric operations

---

## 📈 Visualization

The project includes a Python visualization tool:

```bash
cd visual
python plot.py
```

Uses:

- `map.json`
- `nodes_and_edges.dat`

---

## 🧪 Testing

```bash
cargo test
```

---

## ⚠️ Notes & Limitations

- Focused on **topological correctness**
- Geometry embedding is optional
- API may evolve (research-oriented)

---

## 📈 Roadmap

- [ ] Improve documentation
- [ ] Add higher-level mesh abstractions
- [ ] Performance optimizations
- [ ] More examples and demos
- [ ] Integration with graphics pipelines

---

## 🤝 Contributing

1. Fork the repo  
2. Create a branch  
3. Commit changes  
4. Open a PR  

---

## 🐛 Issues

Report bugs or ideas:

https://github.com/evnekdev/gmap/issues

---

## 📄 License

MIT

---

## 📬 Contact

**Evgenii Nekhoroshev**  
https://github.com/evnekdev
