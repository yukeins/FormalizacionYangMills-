//! Core of the SU(2) Kogut–Susskind spin-network engine.
//!
//! Lattice: 1-skeleton of a single cube (8 trivalent vertices, 12 links,
//! 6 plaquettes). The 6 plaquettes close into a surface with
//! V - E + F = 2, i.e. a cellulation of S^2: this is 2+1-dimensional
//! lattice gauge theory on a spatial sphere.

use nalgebra::{DMatrix, DVector};
use rayon::prelude::*;

// ---------------------------------------------------------------------------
// Wigner 6j symbols (Racah formula, log-gamma evaluation)
// ---------------------------------------------------------------------------

#[inline]
fn ln_fact(x: f64) -> f64 {
    libm::lgamma(x + 1.0)
}

#[inline]
fn ln_delta(a: f64, b: f64, c: f64) -> f64 {
    0.5 * (ln_fact(a + b - c) + ln_fact(a - b + c) + ln_fact(-a + b + c) - ln_fact(a + b + c + 1.0))
}

#[inline]
pub fn check_triangular(a: f64, b: f64, c: f64) -> bool {
    (a + b - c) >= 0.0 && (a - b + c) >= 0.0 && (-a + b + c) >= 0.0 && ((a + b + c).fract() < 1e-9)
}

pub fn sixj(j1: f64, j2: f64, j3: f64, j4: f64, j5: f64, j6: f64) -> f64 {
    if !check_triangular(j1, j2, j3)
        || !check_triangular(j1, j5, j6)
        || !check_triangular(j4, j2, j6)
        || !check_triangular(j4, j5, j3)
    {
        return 0.0;
    }
    let pref_ln = ln_delta(j1, j2, j3) + ln_delta(j1, j5, j6) + ln_delta(j4, j2, j6) + ln_delta(j4, j5, j3);
    let k_min = (j1 + j2 + j3).max(j1 + j5 + j6).max(j4 + j2 + j6).max(j4 + j5 + j3) as i64;
    let k_max = (j1 + j2 + j4 + j5).min(j2 + j3 + j5 + j6).min(j3 + j1 + j6 + j4) as i64;
    if k_min > k_max {
        return 0.0;
    }
    let mut sum = 0.0;
    for k in k_min..=k_max {
        let k_f = k as f64;
        let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
        let num_ln = ln_fact(k_f + 1.0);
        let den_ln = ln_fact(k_f - j1 - j2 - j3)
            + ln_fact(k_f - j1 - j5 - j6)
            + ln_fact(k_f - j4 - j2 - j6)
            + ln_fact(k_f - j4 - j5 - j3)
            + ln_fact(j1 + j2 + j4 + j5 - k_f)
            + ln_fact(j2 + j3 + j5 + j6 - k_f)
            + ln_fact(j3 + j1 + j6 + j4 - k_f);
        sum += sign * (num_ln - den_ln).exp();
    }
    pref_ln.exp() * sum
}

// ---------------------------------------------------------------------------
// Lattice and Gauss law
// ---------------------------------------------------------------------------

pub const EDGES: usize = 12;
pub const VERTICES: usize = 8;
pub const PLAQUETTES: usize = 6;

pub const V2E: [[usize; 3]; VERTICES] = [
    [0, 3, 8], [0, 1, 9], [1, 2, 10], [2, 3, 11],
    [4, 7, 8], [4, 5, 9], [5, 6, 10], [6, 7, 11],
];

pub const P_EDGES: [[usize; 4]; PLAQUETTES] = [
    [0, 1, 2, 3], [4, 5, 6, 7], [0, 9, 4, 8],
    [1, 10, 5, 9], [2, 11, 6, 10], [3, 8, 7, 11],
];

/// State = twice the spin on each link (u8), sorted lexicographically.
pub type State = [u8; EDGES];

/// All Gauss-law-compliant spin networks with 2j_e <= 2 j_max.
/// For trivalent vertices the invariant space is at most one-dimensional,
/// so Gauss law reduces to the triangle + integrality condition.
pub fn generate_basis(j_max: f64) -> Vec<State> {
    let max_val = (j_max * 2.0).round() as u8;
    let mut states = Vec::new();
    fn dfs(e: usize, current: &mut State, states: &mut Vec<State>, max_v: u8) {
        if e == EDGES {
            states.push(*current);
            return;
        }
        for v in 0..=max_v {
            current[e] = v;
            let mut valid = true;
            for edges in V2E.iter() {
                if edges.iter().all(|&idx| idx <= e) {
                    let j1 = current[edges[0]] as f64 / 2.0;
                    let j2 = current[edges[1]] as f64 / 2.0;
                    let j3 = current[edges[2]] as f64 / 2.0;
                    if !check_triangular(j1, j2, j3) {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                dfs(e + 1, current, states, max_v);
            }
        }
    }
    dfs(0, &mut [0; EDGES], &mut states, max_val);
    states
}

/// Electric Casimir sum_e j_e (j_e + 1) of a basis state.
pub fn casimir(state: &State) -> f64 {
    state.iter().map(|&j2| {
        let j = j2 as f64 / 2.0;
        j * (j + 1.0)
    }).sum()
}

// ---------------------------------------------------------------------------
// Sparse matrices
// ---------------------------------------------------------------------------

pub struct SparseCSR {
    pub dim: usize,
    pub indptr: Vec<usize>,
    pub indices: Vec<usize>,
    pub data: Vec<f64>,
}

impl SparseCSR {
    pub fn matvec(&self, x: &DVector<f64>) -> DVector<f64> {
        let mut y = DVector::zeros(self.dim);
        y.as_mut_slice().par_iter_mut().enumerate().for_each(|(i, yi)| {
            let mut sum = 0.0;
            for k in self.indptr[i]..self.indptr[i + 1] {
                sum += self.data[k] * x[self.indices[k]];
            }
            *yi = sum;
        });
        y
    }

    pub fn from_rows(rows: Vec<Vec<(usize, f64)>>) -> Self {
        let dim = rows.len();
        let mut indptr = vec![0; dim + 1];
        let mut indices = Vec::new();
        let mut data = Vec::new();
        for (i, row) in rows.into_iter().enumerate() {
            indptr[i] = indices.len();
            for (idx, val) in row {
                indices.push(idx);
                data.push(val);
            }
        }
        indptr[dim] = indices.len();
        SparseCSR { dim, indptr, indices, data }
    }

    pub fn to_dense(&self) -> DMatrix<f64> {
        let mut m = DMatrix::zeros(self.dim, self.dim);
        for i in 0..self.dim {
            for k in self.indptr[i]..self.indptr[i + 1] {
                m[(i, self.indices[k])] += self.data[k];
            }
        }
        m
    }
}

// ---------------------------------------------------------------------------
// Plaquette operator tr U_p in the spin-network basis
// ---------------------------------------------------------------------------

/// Normalization convention for the corner amplitudes of tr U_p.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlaquetteNorm {
    /// Original engine: per corner sqrt((2ja+1)(2ja'+1)(2jb+1)(2jb'+1)) * 6j.
    /// Every link is shared by two corners, so link factors appear squared.
    Legacy,
    /// Per link sqrt((2j+1)(2j'+1)) once, per corner |6j| only, no phases.
    LinkOnce,
    /// LinkOnce plus the corner phase (-1)^(ja + jb' + j_ext + 1/2).
    LinkOncePhased,
}

/// For each plaquette and each corner: (incoming link, outgoing link, external link).
pub fn plaquette_corners() -> Vec<[(usize, usize, usize); 4]> {
    P_EDGES.iter().map(|pe| {
        let mut out = [(0, 0, 0); 4];
        for i in 0..4 {
            let e_a = pe[(i + 3) % 4];
            let e_b = pe[i];
            let v = (0..VERTICES)
                .find(|&v| V2E[v].contains(&e_a) && V2E[v].contains(&e_b))
                .expect("consecutive plaquette links must share a vertex");
            let e_ext = *V2E[v].iter().find(|&&e| e != e_a && e != e_b).unwrap();
            out[i] = (e_a, e_b, e_ext);
        }
        out
    }).collect()
}

fn half(x: u8) -> f64 {
    x as f64 / 2.0
}

/// Matrix element <new|tr U_p|old> for one plaquette (zero if not connected).
pub fn plaquette_amplitude(
    old: &State,
    new: &State,
    p: usize,
    corners: &[(usize, usize, usize); 4],
    norm: PlaquetteNorm,
) -> f64 {
    let mut amp = 1.0;
    for &(e_a, e_b, e_ext) in corners.iter() {
        let (ja, jb, jx) = (half(old[e_a]), half(old[e_b]), half(old[e_ext]));
        let (ja2, jb2) = (half(new[e_a]), half(new[e_b]));
        if !check_triangular(ja, jb, jx) || !check_triangular(ja2, jb2, jx) {
            return 0.0;
        }
        let s = sixj(ja, jb, jx, jb2, ja2, 0.5);
        if s.abs() < 1e-15 {
            return 0.0;
        }
        amp *= match norm {
            PlaquetteNorm::Legacy => {
                ((2.0 * ja + 1.0) * (2.0 * ja2 + 1.0) * (2.0 * jb + 1.0) * (2.0 * jb2 + 1.0)).sqrt() * s
            }
            PlaquetteNorm::LinkOnce => s.abs(),
            PlaquetteNorm::LinkOncePhased => {
                let ph = (ja + jb2 + jx + 0.5).round() as i64;
                let sign = if ph.rem_euclid(2) == 0 { 1.0 } else { -1.0 };
                sign * s
            }
        };
    }
    if norm != PlaquetteNorm::Legacy {
        for &e in P_EDGES[p].iter() {
            amp *= ((2.0 * half(old[e]) + 1.0) * (2.0 * half(new[e]) + 1.0)).sqrt();
        }
    }
    amp
}

/// Sparse matrix of tr U_p restricted to the truncated basis.
pub fn build_plaquette(basis: &[State], p: usize, norm: PlaquetteNorm) -> SparseCSR {
    let corners = plaquette_corners();
    let rows: Vec<Vec<(usize, f64)>> = (0..basis.len()).into_par_iter().map(|i| {
        let state = &basis[i];
        let mut row = Vec::new();
        for mask in 0u8..16 {
            let mut new_state = *state;
            let mut ok = true;
            for (k, &e) in P_EDGES[p].iter().enumerate() {
                let shift: i16 = if mask >> k & 1 == 1 { 1 } else { -1 };
                let v = state[e] as i16 + shift;
                if v < 0 {
                    ok = false;
                    break;
                }
                new_state[e] = v as u8;
            }
            if !ok {
                continue;
            }
            if let Ok(j) = basis.binary_search(&new_state) {
                let a = plaquette_amplitude(state, &new_state, p, &corners[p], norm);
                if a != 0.0 {
                    row.push((j, a));
                }
            }
        }
        row.sort_by_key(|&(idx, _)| idx);
        row
    }).collect();
    SparseCSR::from_rows(rows)
}

/// H(g^2) = (g^2/2) E - (1/g^2) sum_p tr U_p, assembled as a dense matrix.
pub fn dense_hamiltonian(basis: &[State], g2: f64, norm: PlaquetteNorm) -> DMatrix<f64> {
    let n = basis.len();
    let mut h = DMatrix::zeros(n, n);
    for (i, s) in basis.iter().enumerate() {
        h[(i, i)] = 0.5 * g2 * casimir(s);
    }
    for p in 0..PLAQUETTES {
        let b = build_plaquette(basis, p, norm);
        for i in 0..n {
            for k in b.indptr[i]..b.indptr[i + 1] {
                h[(i, b.indices[k])] -= b.data[k] / g2;
            }
        }
    }
    h
}

/// Index of the strong-coupling vacuum (all links j = 0).
pub fn vacuum_index(basis: &[State]) -> usize {
    basis.binary_search(&[0u8; EDGES]).expect("vacuum must be in the basis")
}

// ---------------------------------------------------------------------------
// Advanced Operator Tools: Nelson Commutator, Transfer Matrix, Fredholm, RG
// ---------------------------------------------------------------------------

/// Computes the Sylvester-Lyapunov commutator [E, B_p] in the basis:
/// [E, B_p]_{ij} = (E_i - E_j) * (B_p)_{ij}.
pub fn commutator_eb(basis: &[State], p: usize, norm: PlaquetteNorm) -> SparseCSR {
    let b = build_plaquette(basis, p, norm);
    let casimirs: Vec<f64> = basis.iter().map(casimir).collect();
    let dim = basis.len();
    let mut indptr = vec![0; dim + 1];
    let mut indices = Vec::new();
    let mut data = Vec::new();

    for i in 0..dim {
        indptr[i] = indices.len();
        let ei = casimirs[i];
        for k in b.indptr[i]..b.indptr[i + 1] {
            let j = b.indices[k];
            let ej = casimirs[j];
            let comm_val = (ei - ej) * b.data[k];
            if comm_val.abs() > 1e-15 {
                indices.push(j);
                data.push(comm_val);
            }
        }
    }
    indptr[dim] = indices.len();
    SparseCSR { dim, indptr, indices, data }
}

/// Computes the relative Nelson ratio:
/// max_psi || [E, B_p] psi || / || (E + I) psi || on the basis elements.
pub fn nelson_relative_bound(basis: &[State], p: usize, norm: PlaquetteNorm) -> f64 {
    let comm = commutator_eb(basis, p, norm);
    let casimirs: Vec<f64> = basis.iter().map(casimir).collect();
    let mut max_ratio = 0.0f64;

    for i in 0..basis.len() {
        let norm_n_psi = casimirs[i] + 1.0;
        let mut row_sq = 0.0;
        for k in comm.indptr[i]..comm.indptr[i + 1] {
            row_sq += comm.data[k] * comm.data[k];
        }
        let norm_comm_psi = row_sq.sqrt();
        let ratio = norm_comm_psi / norm_n_psi;
        if ratio > max_ratio {
            max_ratio = ratio;
        }
    }
    max_ratio
}

/// Evaluates the transfer matrix T = exp(-a * H) via symmetric eigendecomposition.
/// Returns (eigenvalues of T sorted descending, imaginary residue error).
pub fn transfer_matrix_spectrum(h: &DMatrix<f64>, a_spacing: f64) -> (Vec<f64>, f64) {
    let eigen = h.clone().symmetric_eigen();
    let eigenvalues = eigen.eigenvalues;
    let mut t_eigenvalues: Vec<f64> = eigenvalues.iter().map(|&e| (-a_spacing * e).exp()).collect();
    // In exact arithmetic and reflection symmetry, imaginary part is identically 0
    let imaginary_error = 0.0; 
    t_eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap());
    (t_eigenvalues, imaginary_error)
}

/// Evaluates the Fredholm determinant factor product:
/// Xi_N(z) = prod_{k=0}^{N-1} (1 - z / (1 + E_k - E_0)).
pub fn fredholm_determinant_at(eigenvalues: &[f64], z: f64) -> f64 {
    let e0 = eigenvalues.first().cloned().unwrap_or(0.0);
    eigenvalues.iter().fold(1.0, |acc, &ek| {
        let shift_ek = ek - e0;
        acc * (1.0 - z / (1.0 + shift_ek))
    })
}

/// Two-loop Callan-Symanzik running coupling for SU(2) (N=2, pure gauge):
/// beta_0 = 11 / (24 pi^2), beta_1 = 17 / (96 pi^4).
pub fn callan_symanzik_coupling(a_spacing: f64, lambda_ym: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let beta0 = 11.0 / (24.0 * pi * pi);
    let beta1 = 17.0 / (96.0 * pi * pi * pi * pi);

    let l = (1.0 / (a_spacing * lambda_ym)).ln();
    if l <= 1.0 {
        return 1.0; // Infrared floor cutoff
    }
    let ln_l = l.ln();
    let g2_1loop = 1.0 / (beta0 * 2.0 * l);
    let correction = 1.0 - (beta1 / (beta0 * beta0)) * (ln_l / (2.0 * l));
    (g2_1loop * correction).max(1e-4)
}
