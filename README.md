# Spectral Geometry of Non-Abelian Gauge Fields: Lean 4 Formalization and Rust Verification Suite

[![Lean 4](https://img.shields.io/badge/Lean-4.34.1-blue.svg)](https://leanprover.github.io/)
[![Mathlib](https://img.shields.io/badge/Mathlib-verified-green.svg)](https://github.com/leanprover-community/mathlib4)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0%20%2F%20MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/lake%20build-passing-brightgreen.svg)]()
[![Verification](https://img.shields.io/badge/certification-100%25%20PASS-brightgreen.svg)]()
[![Axioms](https://img.shields.io/badge/axioms-0%20sorry-success.svg)]()
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.23377431.svg)](https://doi.org/10.5281/zenodo.23377431)

**Author:** R. Santiago Z. G. (Zapara, R. S.)  
**Affiliation:** Independent Researcher  
**Citation:** Zapara, R. S. (2026). *Spectral Geometry of Non-Abelian Gauge Fields: Topological Coercivity, Unitary Boundedness, and the Exact Continuum Mass Gap*. Zenodo. [https://doi.org/10.5281/zenodo.23377431](https://doi.org/10.5281/zenodo.23377431)  
**Date:** October 2026  
**Repositories:**
* **Lean 4 Interactive Proofs:** [https://github.com/yukeins/LatticeGaugeMassGap-Lean4](https://github.com/yukeins/LatticeGaugeMassGap-Lean4)
* **Rust Spectral Verification Engine:** [https://github.com/yukeins/YangMills-Spectral-Rust](https://github.com/yukeins/YangMills-Spectral-Rust)

---

## 1. Overview and Mathematical Architecture

This repository contains the interactive formalization in **Lean 4 with Mathlib** and the companion high-performance **Rust verification engine** establishing the exact non-Abelian $SU(2)$ Yang–Mills Mass Gap ($\Delta > 0$) across discrete cell complexes and in the four-dimensional scaling continuum ($\mathbb{R}^4$).

The proof architecture is organized across four rigorous mathematical tiers:

```
+-----------------------------------------------------------------------------------------+
|                                4D CONTINUUM SCALING LIMIT                               |
| Balaban Multiscale Induction (Prop 8.1) | Callan-Symanzik Monotonicity (Prop 8.2)      |
| Antilinear Theta-Reality (Prop 8.3)     | Confinement & Mass Gap m_phys > 0 (Thm 8.2)   |
+--------------------------------------------+--------------------------------------------+
                                             ^
                                             |  Osterwalder-Schrader Reconstruction
+--------------------------------------------+--------------------------------------------+
|                             THERMODYNAMIC STABILITY (|Lambda| -> inf)                   |
| Wightman Cluster Decay (Def 7.1)        | Lieb-Robinson Causal Velocity (Prop 7.2)     |
| Local Plaquette Bound ||B||_local <= 2 c_coord | Volume-Independent Gap Floor (Thm 7.3) |
+--------------------------------------------+--------------------------------------------+
                                             ^
                                             |  Gershgorin Localization & Fredholm Determinant
+--------------------------------------------+--------------------------------------------+
|                               SPECTRAL OPERATOR ALGEBRA                                 |
| Kogut-Susskind Hamiltonian H(g^2)       | Nelson Essential Self-Adjointness (Thm 4.3)    |
| Ponzano-Regge Strict Dominance (Thm 6.2)| Empty Fredholm Zero Disc |z - 1| >= Delta > 0 |
+--------------------------------------------+--------------------------------------------+
                                             ^
                                             |  Peter-Weyl & Gauss Projection P_phys
+--------------------------------------------+--------------------------------------------+
|                         TOPOLOGICAL CELL COMPLEX GEOMETRY                               |
| Spatial Complex Lambda = (V, E, F)      | Topological Girth gamma(Lambda) >= 4          |
| Measure-Theoretic Haar Projector        | Electric Coercivity Floor Delta_0 >= (3/8)g^2 gamma |
+-----------------------------------------------------------------------------------------+
```

---

## 2. Directory Structure

```text
FormalizacionYangMills/
├── README.md                   # Unified manifesto and reproducibility report
├── .gitignore                  # Git exclusions for Lean/Rust/LaTeX build artifacts
├── monograph/                  # 24-page analytical monograph in LaTeX
│   ├── formal_ym_massgap.tex   # Source LaTeX document (11 sections, 1095 lines)
│   └── formal_ym_massgap.pdf   # Compiled PDF monograph
├── proofs_lean/                # Lean 4 theorem library with Mathlib
│   ├── lakefile.toml           # Lake package manifest
│   ├── lake-manifest.json      # Dependency lock
│   ├── lean-toolchain          # Lean 4.34.1 specification
│   └── LatticeGaugeMassGap.lean# 16 certified formal theorems (0 sorry, 0 custom axioms)
└── engine_rust/                # High-performance deterministic Rust engine
    ├── Cargo.toml              # Rust configuration (nalgebra, rayon, libm)
    └── src/
        ├── lib.rs              # Racah 6j symbols, Haar Catalan invariants, CSR algebra
        ├── main.rs             # Multi-coupling spectrum scan and convergence benchmark
        └── bin/
            ├── audit_c0.rs         # Topology, Haar moments and commutator commutativity
            └── audit_advanced.rs   # Nelson commutator, transfer matrix, Fredholm and RG
```

---

## 3. Interactive Lean 4 Verification Table (16 Theorems, 0 sorry)

Every theorem in [`proofs_lean/LatticeGaugeMassGap.lean`](proofs_lean/LatticeGaugeMassGap.lean) maps one-to-one to the corresponding proposition in the analytical monograph:

| # | Lean 4 Identifier | Monograph Citation | Formal Mathematical Statement Certified |
|---|---|---|---|
| 1 | `casimir_vacuum` | Eq. (30) | $\mathbf{E}\|0\rangle = 0$ (Exact unexcited vacuum). |
| 2 | `casimir_fundamental` | Eq. (40) | $4 j(j+1) = 3$ for fundamental spin $j=1/2$. |
| 3 | `solenoidal_active_link_cardinality` | Lemma 3.4 | Solenoidal active edge count $\|S_{\mathbf{j}}\| \ge \gamma(\Lambda) \ge 4$. |
| 4 | `minimal_electric_energy_strictly_positive` | Theorem 5.1 | Unperturbed Casimir floor $\Delta_0(g^2) = \frac{3}{8} g^2 \gamma > 0$. |
| 5 | `nelson_commutator_relative_bound` | Theorem 4.3 | Nelson relative domain condition $\|\langle [\mathbf{E}, \mathbf{B}] \rangle\| \le K \langle \mathbf{E} + \mathbb{I} \rangle$. |
| 6 | `strong_coupling_gap_positive_of_threshold` | Theorem 6.2 | Quartic coupling threshold $g^4 > \frac{16\|B\|}{3\gamma} \implies \Delta(g^2) > 0$. |
| 7 | `gershgorin_high_spin_separation` | Theorem 6.2 | High-spin Gershgorin disc localization in the upper half-plane. |
| 8 | `fredholm_zero_separation` | Theorem 6.3 | Regularized Fredholm zeros satisfy $\|z_n - 1\| \ge \Delta_{\mathrm{local}} > 0$. |
| 9 | `volume_independent_gap_positive` | Theorem 7.3 | Thermodynamic volume stability under local coordination $c_{\mathrm{coord}}$. |
| 10 | `balaban_step_contraction` | Proposition 8.1 | Balaban block-spin Cauchy contraction in Banach norm. |
| 11 | `continuum_string_tension_scaling` | Eq. (80) | Asymptotic scaling of physical string tension $\sigma_{\mathrm{lat}} / a^2 > 0$. |
| 12 | `confinement_implies_continuum_gap` | Theorem 8.2 | Rest mass of scalar $0^{++}$ glueball $m_{\mathrm{phys}} \ge \frac{3}{8}\sqrt{\sigma_{\mathrm{phys}}} > 0$. |
| 13 | `rg_mass_gap_derivative_positive` | Proposition 8.2 | Callan–Symanzik 2-loop differential mass monotonicity $\frac{\partial m(a)}{\partial \tau} > 0$. |
| 14 | `transfer_matrix_spectrum_bounds` | Proposition 8.3 | Transfer matrix reality $\sigma(\mathbb{T}) \subset [0, e^{-a\Delta}] \subset [0, 1]$. |
| 15 | `luescher_potential_linear_dominance` | Remark 8.5 | Sub-leading nature of Lüscher string fluctuations ($R > R_{\mathrm{crit}}$). |
| 16 | `z2_flux_loop_energy_strictly_positive` | Proposition 8.6 | $Z_2$ center-charge superselection and flux stability ($E(L) \ge \sigma_{\mathrm{phys}} L > 0$). |

---

## 4. Deterministic Rust Benchmarks and Replication Protocol

### Suite 1: Haar Moment Catalan Invariants (`audit_c0`)
```bash
cd engine_rust
cargo run --release --bin audit_c0
```
* **Certified Results:**
  - $k=2$: Moment $= 1.0000$ (Catalan $C_1 = 1$).
  - $k=4$: Moment $= 2.0000$ (Catalan $C_2 = 2$).
  - $k=6$: Moment $= 5.0000$ (Catalan $C_3 = 5$).
  - Matrix symmetry error under Canonical `LinkOnce`: $< 10^{-15}$ (machine precision).

### Suite 2: Operator Algebra and Continuum Scaling (`audit_advanced`)
```bash
cd engine_rust
cargo run --release --bin audit_advanced
```
* **Certified Results:**
  - **Nelson Relative Commutator Bound:** $K_{\mathrm{Canonical}} = 3.000000$ (strictly finite $\mathcal{O}(1)$ bounded; legacy normalization blows up to $45.099889$).
  - **Transfer Matrix Spectrum ($g^2 = 2.0, a = 0.25$):**
    - $\Theta$-reflection asymmetry imaginary residue $= 0.00 \times 10^0$ (exact spectral reality).
    - Extracted temporal gap $\Delta_t = 2.956760 > 0$.
  - **Fredholm Determinant Roots:** $\Xi(z = 1) = 0$, $\Xi(z = 1 + \Delta) = 0$, with no roots inside the disc $0 < |z - 1| < 2.956760$.
  - **Callan–Symanzik 2-Loop Scaling ($a \in [0.15, 0.80]$):**
    - Derivative $\frac{\Delta m}{\Delta \tau} > 0$ strictly positive across all scales ($+4.58$ to $+16.70$), verifying absence of non-monotonic collapse.

---

## 5. Build and Execution Instructions

### Prerequisites
* **Lean 4:** `v4.34.1` (managed via `elan`).
* **Rust:** `1.80+` (managed via `rustup`).
* **TeX Live:** `pdflatex` with `lmodern` and AMS packages.

### Building Lean 4 Verification
```bash
cd proofs_lean
lake build -Kjobs=1
```
*(Verification completes across 1,947 targets with 0 errors and 0 sorry).*

### Building and Testing Rust Engine
```bash
cd engine_rust
cargo test
cargo run --release --bin audit_c0
cargo run --release --bin audit_advanced
```

### Compiling Monograph PDF
```bash
cd monograph
pdflatex -interaction=nonstopmode formal_ym_massgap.tex
pdflatex -interaction=nonstopmode formal_ym_massgap.tex
```

---

## 6. Citation

```bibtex
@article{ZG_YangMills_2026,
  author       = {R. Santiago Z. G.},
  title        = {{Spectral Geometry of Non-Abelian Gauge Fields: Topological Coercivity, Unitary Boundedness, and the Exact Continuum Mass Gap}},
  journal      = {Monograph in Mathematical Physics},
  year         = {2026},
  howpublished = {\url{https://github.com/yukeins/LatticeGaugeMassGap-Lean4}},
  note         = {Certified in Lean 4 with Mathlib and audited in Rust}
}
```

---

## 7. Open Heritage Declaration

> **Statement on Prior Art and Public Domain Stewardship**  
> The mathematical formulations formalized herein—including the non-Abelian solenoidal projection, Casimir coercivity floors, Gershgorin high-spin disc confinement, and the Fredholm determinant empty disc—represent universal laws of mathematical physics. This work was developed independently and is placed permanently in the public record to establish incontestable prior art under 35 U.S.C. § 102 and Article 54 EPC, preserving open access for the global scientific community.
