//! Cornering cycles C0/C1/C5 on the legacy engine.
//!
//! Each check compares the engine against an exact, representation-independent
//! fact about SU(2) lattice gauge theory:
//!   T. topology of the lattice (Euler characteristic, cycle rank, girth);
//!   H. Haar moments  <0|(tr U_p)^{2n}|0> = Catalan(n) = 1, 2, 5, ...
//!      (tr U is a multiplication operator on L^2(SU(2)); its moments in the
//!      vacuum are Haar integrals, independent of any basis convention);
//!   C. multiplication operators commute: [tr U_p, tr U_q] |0> = 0;
//!   N. ||tr U_p|| <= 2 (|tr U| <= 2 on SU(2)), inherited by truncations;
//!   S. strong-coupling gap: the lightest gauge-invariant excitation is a
//!      closed flux loop of minimal length (the girth), Delta/g^2 -> (3/8)*girth.

use nalgebra::DVector;
use ym_spectral::*;

fn catalan(n: usize) -> f64 {
    let mut c = 1.0;
    for k in 0..n {
        c = c * 2.0 * (2.0 * k as f64 + 1.0) / (k as f64 + 2.0);
    }
    c
}

fn girth() -> usize {
    // BFS from every vertex; shortest cycle in a simple graph.
    let mut adj = vec![Vec::new(); VERTICES];
    for e in 0..EDGES {
        let ends: Vec<usize> = (0..VERTICES).filter(|&v| V2E[v].contains(&e)).collect();
        adj[ends[0]].push((ends[1], e));
        adj[ends[1]].push((ends[0], e));
    }
    let mut best = usize::MAX;
    for s in 0..VERTICES {
        let mut dist = vec![usize::MAX; VERTICES];
        let mut via = vec![usize::MAX; VERTICES];
        dist[s] = 0;
        let mut q = std::collections::VecDeque::from([s]);
        while let Some(u) = q.pop_front() {
            for &(w, e) in &adj[u] {
                if dist[w] == usize::MAX {
                    dist[w] = dist[u] + 1;
                    via[w] = e;
                    q.push_back(w);
                } else if via[u] != e {
                    best = best.min(dist[u] + dist[w] + 1);
                }
            }
        }
    }
    best
}

fn unit(n: usize, i: usize) -> DVector<f64> {
    let mut v = DVector::zeros(n);
    v[i] = 1.0;
    v
}

fn main() {
    println!("== T. Topology of the lattice ==");
    let (v, e, f) = (VERTICES as i64, EDGES as i64, PLAQUETTES as i64);
    let g = girth();
    println!("  V={v} E={e} F={f}  chi = V-E+F = {}  (2 => sphere S^2)", v - e + f);
    println!("  cycle rank E-V+1 = {}  (independent closed loops; plaquettes give F-1 = {})", e - v + 1, f - 1);
    println!("  girth (shortest closed loop) = {g}");
    println!("  first Betti number of the surface b1 = 2 - chi = {}  => no non-contractible loops, no center-flux sectors", 2 - (v - e + f));

    println!("\n== Basis dimensions (Gauss-invariant spin networks) ==");
    for jm in [0.5, 1.0, 1.5] {
        println!("  j_max = {jm:<4} dim = {}", generate_basis(jm).len());
    }

    let basis = generate_basis(1.5);
    let n = basis.len();
    let vac = vacuum_index(&basis);
    let modes = [PlaquetteNorm::Legacy, PlaquetteNorm::LinkOnce, PlaquetteNorm::LinkOncePhased];

    println!("\n== H. Haar moments <0|(tr U_p)^k|0> on j_max = 3/2 (exact up to k = 6) ==");
    println!("  {:<16} {:>10} {:>10} {:>10} {:>10}", "convention", "k=2", "k=4", "k=6", "sym.err");
    println!("  {:<16} {:>10.4} {:>10.4} {:>10.4}", "exact (Catalan)", catalan(1), catalan(2), catalan(3));
    for &m in &modes {
        let b = build_plaquette(&basis, 0, m);
        let d = b.to_dense();
        let sym = (&d - d.transpose()).abs().max();
        let mut w = unit(n, vac);
        let mut mom = Vec::new();
        for k in 1..=6 {
            w = b.matvec(&w);
            if k % 2 == 0 {
                mom.push(w[vac]);
            }
        }
        println!("  {:<16} {:>10.4} {:>10.4} {:>10.4} {:>10.2e}", format!("{m:?}"), mom[0], mom[1], mom[2], sym);
    }

    println!("\n== C. Commutation of plaquettes on the vacuum (p=0 with adjacent q=2, opposite q=1) ==");
    for &m in &modes {
        let b0 = build_plaquette(&basis, 0, m);
        let b1 = build_plaquette(&basis, 1, m);
        let b2 = build_plaquette(&basis, 2, m);
        let v0 = unit(n, vac);
        let c02 = (b0.matvec(&b2.matvec(&v0)) - b2.matvec(&b0.matvec(&v0))).norm();
        let x = b2.matvec(&b0.matvec(&b2.matvec(&b0.matvec(&v0))));
        let y = b1.matvec(&b1.matvec(&b0.matvec(&b0.matvec(&v0))));
        println!(
            "  {:<16} |[B0,B2]0| = {:.2e}   <0|B0B2B0B2|0> = {:.4} (exact 1)   <0|B0^2B1^2|0> = {:.4} (exact 1)",
            format!("{m:?}"), c02, x[vac], y[vac]
        );
    }

    println!("\n== N. Spectral radius of a single truncated plaquette (must be <= 2) ==");
    let small = generate_basis(1.0);
    for &m in &modes {
        let d = build_plaquette(&small, 0, m).to_dense();
        let ev = d.symmetric_eigen().eigenvalues;
        let r = ev.iter().fold(0.0f64, |a, &x| a.max(x.abs()));
        println!("  {:<16} max|eig| = {:.4}", format!("{m:?}"), r);
    }

    println!("\n== S. Spectrum of H(g^2) on j_max = 1 (dense, all sectors) ==");
    println!("  {:<16} {:>7} {:>12} {:>12} {:>14}", "convention", "g^2", "Delta", "Delta/g^2", "(3/8)*girth");
    for &m in &modes {
        for g2 in [0.5, 2.0, 10.0, 100.0] {
            let ev = dense_hamiltonian(&small, g2, m).symmetric_eigen().eigenvalues;
            let mut s: Vec<f64> = ev.iter().cloned().collect();
            s.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let gap = s[1] - s[0];
            println!("  {:<16} {:>7} {:>12.5} {:>12.5} {:>14.5}", format!("{m:?}"), g2, gap, gap / g2, 0.375 * g as f64);
        }
    }
}
