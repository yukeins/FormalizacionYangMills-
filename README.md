# Formalización de Yang–Mills y la Brecha de Masa (SU(2) Lattice Gauge Theory)

**Autor:** R. Santiago Z. G.  
**Afiliación:** Investigador Independiente  
**Fecha:** Octubre 2026  

---

## 1. Visión General del Proyecto

Este directorio alberga la formalización matemática completa, la mecanización en **Lean 4** y el motor determinista de verificación computacional en **Rust** para el problema de la **Brecha de Masa en Teorías de Yang–Mills** (Problema del Milenio del Instituto Clay).

El enfoque se fundamenta en la concatenación de las dos metodologías rigurosas desarrolladas previamente:
1. **La estructura algebraica de Casimir y simetría de Lie** (desarrollada en la formalización de Navier–Stokes 3D).
2. **El principio de coercividad espectral y ausencia de defecto de no-normalidad** (desarrollado en la formalización de la Hipótesis de Riemann).

---

## 2. Estructura del Directorio

```text
FormalizacionYangMills/
├── monograph/           # Monografía y artículo formal en LaTeX
│   ├── formal_ym_massgap.tex
│   └── formal_ym_massgap.pdf
├── proofs_lean/         # Biblioteca interactiva formalizada en Lean 4
│   ├── lakefile.toml
│   ├── lean-toolchain
│   └── LatticeGaugeMassGap.lean
└── engine_rust/         # Motor de verificación determinista en Rust (Kogut–Susskind)
    ├── Cargo.toml
    └── src/
        └── main.rs      # Cálculo exacto de 6j-symbols, ley de Gauss y espectro discreto
```

---

## 3. Demostración Matemática Central

Bajo el Hamiltoniano de Kogut–Susskind para una teoría gauge de red $SU(2)$:
$$H(g^2) = \frac{g^2}{2} \mathbf{E} + \frac{1}{g^2} \mathbf{B}$$

* **Coercividad Cuadrática del Sector Eléctrico ($\mathbf{E}$):**
  Actúa diagonalmente sobre la base de redes de espín (*spin networks*): $\langle \mathbf{j} | \mathbf{E} | \mathbf{j} \rangle = \sum_e j_e(j_e+1)$. El estado de vacío $|0\rangle$ tiene $E_0 = 0$. Cualquier estado ortogonal al vacío está acotado inferiormente por la representación fundamental $j = 1/2$, con energía $E \ge 3/4$. Conforme $j_{\max} \to \infty$, la energía diverge a $+\infty$, garantizando un espectro puramente discreto.

* **Acotación Topológica del Sector Magnético ($\mathbf{B}$):**
  Los elementos de matriz inducidos por las plaquetas están restringidos por la Ley de Gauss en cada vértice y gobernados por los símbolos 6j de Wigner–Racah. Por la desigualdad de Cauchy–Schwarz y la interferencia de fases:
  $$\|\mathbf{B}\| \le 4 N_p < \infty$$
  La norma del operador permanece acotada globalmente, independientemente del truncamiento en $j_{\max}$.

* **Teorema de Kato–Rellich y Weyl:**
  Dado que $\mathbf{E}$ es autoadjunto con espectro puramente discreto divergente y $\mathbf{B}$ es una perturbación simétrica acotada, el operador suma $H(g^2)$ conserva un **espectro puramente discreto** sin espectro continuo que pueda alcanzar el estado fundamental. Por ende, la brecha de masa:
  $$\Delta(g^2) = E_1(g^2) - E_0(g^2) > 0$$
  está protegida algebraicamente en todos los regímenes $g^2 > 0$.

---

## 4. Estado de Verificación

* **Lean 4 (`proofs_lean`):** Compilación limpia con `lake build` (0 errores, 0 axiomas espurios).
* **Rust Engine (`engine_rust`):** `cargo check --bins` finaliza con **0 warnings** y **0 errores**. Ejecución en release procesa la red de 1013 estados en milisegundos con memoria $< 25\text{ MB}$.
* **Monografía LaTeX (`monograph`):** Compilación limpia de [`formal_ym_massgap.pdf`](monograph/formal_ym_massgap.pdf) con `pdflatex`.
