# ML Practice: Python & Rust Dual Implementations

A repository dedicated to exploring machine learning concepts, algorithms, and models by implementing them side-by-side in both **Python** and **Rust**. 

The main goal of this project is to deepen understanding of ML fundamentals while contrasting Python's high-level ecosystem (flexibility, ease of experimentation) with Rust's high-performance, strongly-typed memory safety features.

---

## 📁 Repository Structure

The project is structured into two main directories:

```text
.
├── pyml/    # Python implementations, scripts, and notebooks
└── ruml/      # Rust implementations and Cargo packages
```

* Every algorithm or model implemented in `pyml/` has a corresponding implementation in `ruml/`.

---

## 🚀 Implemented Algorithms

| Algorithm | Python Implementation | Rust Implementation |
| :--- | :---: | :---: |
| **Linear Regression** | [Python Linear Regression](/pyml/linear_regression.py) | [Rust Linear Regression](/ruml/src/linear_regression.rs) |

*(More algorithms to be added as the repository grows!)*

---

## 🛠️ Getting Started

### Python

1. Navigate to the Python directory:
   ```bash
   cd python
   ```
2. Set up a virtual environment and install dependencies:
   ```bash
   python -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   pip install -r requirements.txt
   ```

### Rust

1. Ensure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install).
2. Navigate to the Rust directory:
   ```bash
   cd rust
   ```
3. Run the projects using Cargo:
   ```bash
   cargo run --bin linear_regression
   ```

---

## 🎯 Future Roadmap

- [ ] Logistic Regression
- [ ] Decision Trees & Random Forests
- [ ] Neural Networks / Multilayer Perceptron (MLP)
- [ ] Benchmarking Python vs. Rust execution performance
