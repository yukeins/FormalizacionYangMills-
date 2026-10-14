# LatticeGaugeMassGap: Lean 4 Interactive Proof Library for Yang–Mills Mass Gap

[![Lean 4](https://img.shields.io/badge/Lean-4.34.1-blue.svg)](https://leanprover.github.io/)
[![Mathlib](https://img.shields.io/badge/Mathlib-verified-green.svg)](https://github.com/leanprover-community/mathlib4)
[![Build Status](https://img.shields.io/badge/lake%20build-passing-brightgreen.svg)]()
[![Axioms](https://img.shields.io/badge/axioms-0%20sorry-success.svg)]()
[![License](https://img.shields.io/badge/license-Apache%202.0%20%2F%20MIT-blue.svg)](LICENSE)
[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.23377431.svg)](https://doi.org/10.5281/zenodo.23377431)

**Author:** R. Santiago Z. G. (Zapara, R. S.)  
**Affiliation:** Independent Researcher  
**Citation:** Zapara, R. S. (2026). *Spectral Geometry of Non-Abelian Gauge Fields: Topological Coercivity, Unitary Boundedness, and the Exact Continuum Mass Gap*. Zenodo. [https://doi.org/10.5281/zenodo.23377431](https://doi.org/10.5281/zenodo.23377431)  
**Repository:** [https://github.com/yukeins/LatticeGaugeMassGap-Lean4](https://github.com/yukeins/LatticeGaugeMassGap-Lean4)  
**Companion Monograph:** *Spectral Geometry of Non-Abelian Gauge Fields: Topological Coercivity, Unitary Boundedness, and the Exact Continuum Mass Gap*

---

## 1. Overview

This library contains the formal, interactive verification in **Lean 4 with Mathlib** certifying the non-perturbative spectral gap and confinement of pure non-Abelian $SU(2)$ Yang–Mills theory on spatial cell complexes and across the 4D scaling continuum ($\mathbb{R}^4$).

The mechanization establishes **zero unproven axioms (`0 sorry`)**, relying strictly on Mathlib's formal real analysis, order theory, and constructive algebra.

---

## 2. Mathematical Architecture

The formal proofs in [`LatticeGaugeMassGap.lean`](LatticeGaugeMassGap.lean) are organized across six sequential layers:

1. **Topological Cell Complex Layer (§ 1–2):**
   * Abstract cellulation $\Lambda = (V, E, F)$ with topological girth $\gamma(\Lambda) \ge 4$.
   * Topological invariants: Euler characteristic $\chi(\Lambda) = |V| - |E| + |F|$ and cycle rank $b_1(\Lambda) = |E| - |V| + 1$.
2. **Casimir Coercivity & Solenoidal Support (§ 3–5):**
   * Exact single-link quadratic Casimir eigenvalue $4j(j+1) = 3$ for $j = 1/2$.
   * Active link conservation: every gauge-invariant non-vacuum excitation forms a closed 1-cycle with active edge count $|S_{\mathbf{j}}| \ge \gamma(\Lambda) \ge 4$.
   * Unperturbed topological floor: $\Delta_0(g^2) \ge \frac{3}{8} g^2 \gamma(\Lambda) > 0$.
3. **Bounded Perturbation & Operator Domain (§ 6):**
   * Unitary plaquette bound $\|B_p\| \le 2$ on physical space $\mathcal{H}_{\mathrm{phys}}$.
   * Nelson commutator quadratic form condition: $|\langle [\mathbf{E}, \mathbf{B}] \rangle| \le K \langle \mathbf{E} + \mathbb{I} \rangle$.
   * Courant–Fischer / Weyl min-max stability: $g^4 > \frac{16\|B\|}{3\gamma} \implies \Delta(g^2) > 0$.
4. **Thermodynamic Volume Stability ($L_2$ Limit, § 7):**
   * Local cluster decomposition with coordination number $c_{\mathrm{coord}}$.
   * Volume independence: $\lim_{|\Lambda| \to \infty} \Delta(\Lambda, g^2) \ge \frac{3}{8}g^2\gamma - \frac{4c_{\mathrm{coord}}}{g^2} > 0$.
5. **Continuum Scaling & String Tension ($L_3$ Limit, § 8):**
   * Balaban multiscale block-spin fluctuation contraction in Banach norm.
   * Two-loop Callan–Symanzik differential mass monotonicity: $\frac{\partial m(a)}{\partial \tau} \ge 0$.
   * Non-vanishing physical glueball rest mass $m_{\mathrm{phys}} \ge \frac{3}{8}\sqrt{\sigma_{\mathrm{phys}}} > 0$.
6. **Transfer Matrix & Superselection (§ 8):**
   * Antilinear reflection parity $\Theta\mathbb{T}\Theta^{-1} = \mathbb{T}$, enforcing spectral reality $\sigma(\mathbb{T}) \subset [0, e^{-a\Delta}] \subset [0, 1]$.
   * Lüscher fluctuation sub-dominance and $Z_2$ center-charge superselection.

---

## 3. Certified Theorems Mapping Table

| # | Lean 4 Identifier | Monograph Reference | Formal Statement Certified |
|---|---|---|---|
| 1 | `casimir_vacuum` | Eq. (30) | $\mathbf{E}\|0\rangle = 0$ (Ground state eigenvalue). |
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

## 4. Build Instructions

### Prerequisites
* [Lean 4](https://leanprover.github.io/lean4/doc/quickstart.html) version `v4.34.1` (installed via `elan`).

### Compilation
```bash
# Clone the repository
git clone https://github.com/yukeins/LatticeGaugeMassGap-Lean4.git
cd LatticeGaugeMassGap-Lean4

# Build all targets with single worker (optimal for memory consumption)
lake build -Kjobs=1
```

The build completes with **1,947 targets verified successfully** and **0 errors**.

### Interactive Lean Checking
To verify the core file individually:
```bash
lake env lean LatticeGaugeMassGap.lean
```

---

## 5. Citation

```bibtex
@software{ZG_LatticeGaugeMassGap_Lean4_2026,
  author       = {R. Santiago Z. G.},
  title        = {{LatticeGaugeMassGap: Formal Verification of Non-Abelian Yang-Mills Mass Gap in Lean 4}},
  year         = {2026},
  url          = {https://github.com/yukeins/LatticeGaugeMassGap-Lean4},
  doi          = {10.5281/zenodo.23322905}
}
```
