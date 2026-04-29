#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::needless_range_loop)]
use super::dense_mlpoly::DensePolynomial;
use super::dense_mlpoly::{
    EqPolynomial, IdentityPolynomial, PolyCommitment, PolyCommitmentGens, PolyEvalProof,
};
use super::errors::ProofVerifyError;
use super::math::Math;
use super::product_tree::{
    DotProductCircuit, ProductCircuit, ProductCircuitEvalProofBatched,
};
use super::random::RandomTape;
use super::scalar::Scalar;
use super::timer::Timer;
use super::transcript::{AppendToTranscript, ProofTranscript};
use core::cmp::Ordering;
use merlin::Transcript;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct SparseMatEntry {
    row: usize,
    col: usize,
    val: Scalar,
}
impl SparseMatEntry {
    pub fn new(row: usize, col: usize, val: Scalar) -> Self {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SparseMatPolynomial {
    num_vars_x: usize,
    num_vars_y: usize,
    M: Vec<SparseMatEntry>,
}
pub struct Derefs {
    row_ops_val: Vec<DensePolynomial>,
    col_ops_val: Vec<DensePolynomial>,
    comb: DensePolynomial,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DerefsCommitment {
    comm_ops_val: PolyCommitment,
}
impl Derefs {
    pub fn new(
        row_ops_val: Vec<DensePolynomial>,
        col_ops_val: Vec<DensePolynomial>,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn commit(&self, gens: &PolyCommitmentGens) -> DerefsCommitment {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DerefsEvalProof {
    proof_derefs: PolyEvalProof,
}
impl DerefsEvalProof {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    fn prove_single(
        joint_poly: &DensePolynomial,
        r: &[Scalar],
        evals: Vec<Scalar>,
        gens: &PolyCommitmentGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> PolyEvalProof {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        derefs: &Derefs,
        eval_row_ops_val_vec: &[Scalar],
        eval_col_ops_val_vec: &[Scalar],
        r: &[Scalar],
        gens: &PolyCommitmentGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    fn verify_single(
        proof: &PolyEvalProof,
        comm: &PolyCommitment,
        r: &[Scalar],
        evals: Vec<Scalar>,
        gens: &PolyCommitmentGens,
        transcript: &mut Transcript,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        r: &[Scalar],
        eval_row_ops_val_vec: &[Scalar],
        eval_col_ops_val_vec: &[Scalar],
        gens: &PolyCommitmentGens,
        comm: &DerefsCommitment,
        transcript: &mut Transcript,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
impl AppendToTranscript for DerefsCommitment {
    fn append_to_transcript(&self, label: &'static [u8], transcript: &mut Transcript) {
        panic!("STUB: not implemented");
    }
}
#[derive(Serialize, Deserialize)]
struct AddrTimestamps {
    ops_addr_usize: Vec<Vec<usize>>,
    ops_addr: Vec<DensePolynomial>,
    read_ts: Vec<DensePolynomial>,
    audit_ts: DensePolynomial,
}
impl AddrTimestamps {
    pub fn new(num_cells: usize, num_ops: usize, ops_addr: Vec<Vec<usize>>) -> Self {
        panic!("STUB: not implemented");
    }
    fn deref_mem(addr: &[usize], mem_val: &[Scalar]) -> DensePolynomial {
        panic!("STUB: not implemented");
    }
    pub fn deref(&self, mem_val: &[Scalar]) -> Vec<DensePolynomial> {
        panic!("STUB: not implemented");
    }
}
#[derive(Serialize, Deserialize)]
pub struct MultiSparseMatPolynomialAsDense {
    batch_size: usize,
    val: Vec<DensePolynomial>,
    row: AddrTimestamps,
    col: AddrTimestamps,
    comb_ops: DensePolynomial,
    comb_mem: DensePolynomial,
}
#[derive(Serialize, Deserialize)]
pub struct SparseMatPolyCommitmentGens {
    gens_ops: PolyCommitmentGens,
    gens_mem: PolyCommitmentGens,
    gens_derefs: PolyCommitmentGens,
}
impl SparseMatPolyCommitmentGens {
    pub fn new(
        label: &'static [u8],
        num_vars_x: usize,
        num_vars_y: usize,
        num_nz_entries: usize,
        batch_size: usize,
    ) -> SparseMatPolyCommitmentGens {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SparseMatPolyCommitment {
    batch_size: usize,
    num_ops: usize,
    num_mem_cells: usize,
    comm_comb_ops: PolyCommitment,
    comm_comb_mem: PolyCommitment,
}
impl AppendToTranscript for SparseMatPolyCommitment {
    fn append_to_transcript(&self, _label: &'static [u8], transcript: &mut Transcript) {
        panic!("STUB: not implemented");
    }
}
impl SparseMatPolynomial {
    pub fn new(num_vars_x: usize, num_vars_y: usize, M: Vec<SparseMatEntry>) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn get_num_nz_entries(&self) -> usize {
        panic!("STUB: not implemented");
    }
    fn sparse_to_dense_vecs(&self, N: usize) -> (Vec<usize>, Vec<usize>, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
    fn multi_sparse_to_dense_rep(
        sparse_polys: &[&SparseMatPolynomial],
    ) -> MultiSparseMatPolynomialAsDense {
        panic!("STUB: not implemented");
    }
    fn evaluate_with_tables(
        &self,
        eval_table_rx: &[Scalar],
        eval_table_ry: &[Scalar],
    ) -> Scalar {
        panic!("STUB: not implemented");
    }
    pub fn multi_evaluate(
        polys: &[&SparseMatPolynomial],
        rx: &[Scalar],
        ry: &[Scalar],
    ) -> Vec<Scalar> {
        panic!("STUB: not implemented");
    }
    pub fn multiply_vec(
        &self,
        num_rows: usize,
        num_cols: usize,
        z: &[Scalar],
    ) -> Vec<Scalar> {
        panic!("STUB: not implemented");
    }
    pub fn compute_eval_table_sparse(
        &self,
        rx: &[Scalar],
        num_rows: usize,
        num_cols: usize,
    ) -> Vec<Scalar> {
        panic!("STUB: not implemented");
    }
    pub fn multi_commit(
        sparse_polys: &[&SparseMatPolynomial],
        gens: &SparseMatPolyCommitmentGens,
    ) -> (SparseMatPolyCommitment, MultiSparseMatPolynomialAsDense) {
        panic!("STUB: not implemented");
    }
}
impl MultiSparseMatPolynomialAsDense {
    pub fn deref(&self, row_mem_val: &[Scalar], col_mem_val: &[Scalar]) -> Derefs {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
struct ProductLayer {
    init: ProductCircuit,
    read_vec: Vec<ProductCircuit>,
    write_vec: Vec<ProductCircuit>,
    audit: ProductCircuit,
}
#[derive(Debug)]
struct Layers {
    prod_layer: ProductLayer,
}
impl Layers {
    fn build_hash_layer(
        eval_table: &[Scalar],
        addrs_vec: &[DensePolynomial],
        derefs_vec: &[DensePolynomial],
        read_ts_vec: &[DensePolynomial],
        audit_ts: &DensePolynomial,
        r_mem_check: &(Scalar, Scalar),
    ) -> (DensePolynomial, Vec<DensePolynomial>, Vec<DensePolynomial>, DensePolynomial) {
        panic!("STUB: not implemented");
    }
    pub fn new(
        eval_table: &[Scalar],
        addr_timestamps: &AddrTimestamps,
        poly_ops_val: &[DensePolynomial],
        r_mem_check: &(Scalar, Scalar),
    ) -> Self {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
struct PolyEvalNetwork {
    row_layers: Layers,
    col_layers: Layers,
}
impl PolyEvalNetwork {
    pub fn new(
        dense: &MultiSparseMatPolynomialAsDense,
        derefs: &Derefs,
        mem_rx: &[Scalar],
        mem_ry: &[Scalar],
        r_mem_check: &(Scalar, Scalar),
    ) -> Self {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct HashLayerProof {
    eval_row: (Vec<Scalar>, Vec<Scalar>, Scalar),
    eval_col: (Vec<Scalar>, Vec<Scalar>, Scalar),
    eval_val: Vec<Scalar>,
    eval_derefs: (Vec<Scalar>, Vec<Scalar>),
    proof_ops: PolyEvalProof,
    proof_mem: PolyEvalProof,
    proof_derefs: DerefsEvalProof,
}
impl HashLayerProof {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    fn prove_helper(
        rand: (&Vec<Scalar>, &Vec<Scalar>),
        addr_timestamps: &AddrTimestamps,
    ) -> (Vec<Scalar>, Vec<Scalar>, Scalar) {
        panic!("STUB: not implemented");
    }
    fn prove(
        rand: (&Vec<Scalar>, &Vec<Scalar>),
        dense: &MultiSparseMatPolynomialAsDense,
        derefs: &Derefs,
        gens: &SparseMatPolyCommitmentGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    fn verify_helper(
        rand: &(&Vec<Scalar>, &Vec<Scalar>),
        claims: &(Scalar, Vec<Scalar>, Vec<Scalar>, Scalar),
        eval_ops_val: &[Scalar],
        eval_ops_addr: &[Scalar],
        eval_read_ts: &[Scalar],
        eval_audit_ts: &Scalar,
        r: &[Scalar],
        r_hash: &Scalar,
        r_multiset_check: &Scalar,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
    fn verify(
        &self,
        rand: (&Vec<Scalar>, &Vec<Scalar>),
        claims_row: &(Scalar, Vec<Scalar>, Vec<Scalar>, Scalar),
        claims_col: &(Scalar, Vec<Scalar>, Vec<Scalar>, Scalar),
        claims_dotp: &[Scalar],
        comm: &SparseMatPolyCommitment,
        gens: &SparseMatPolyCommitmentGens,
        comm_derefs: &DerefsCommitment,
        rx: &[Scalar],
        ry: &[Scalar],
        r_hash: &Scalar,
        r_multiset_check: &Scalar,
        transcript: &mut Transcript,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct ProductLayerProof {
    eval_row: (Scalar, Vec<Scalar>, Vec<Scalar>, Scalar),
    eval_col: (Scalar, Vec<Scalar>, Vec<Scalar>, Scalar),
    eval_val: (Vec<Scalar>, Vec<Scalar>),
    proof_mem: ProductCircuitEvalProofBatched,
    proof_ops: ProductCircuitEvalProofBatched,
}
impl ProductLayerProof {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        row_prod_layer: &mut ProductLayer,
        col_prod_layer: &mut ProductLayer,
        dense: &MultiSparseMatPolynomialAsDense,
        derefs: &Derefs,
        eval: &[Scalar],
        transcript: &mut Transcript,
    ) -> (Self, Vec<Scalar>, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        num_ops: usize,
        num_cells: usize,
        eval: &[Scalar],
        transcript: &mut Transcript,
    ) -> Result<
        (Vec<Scalar>, Vec<Scalar>, Vec<Scalar>, Vec<Scalar>, Vec<Scalar>),
        ProofVerifyError,
    > {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct PolyEvalNetworkProof {
    proof_prod_layer: ProductLayerProof,
    proof_hash_layer: HashLayerProof,
}
impl PolyEvalNetworkProof {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        network: &mut PolyEvalNetwork,
        dense: &MultiSparseMatPolynomialAsDense,
        derefs: &Derefs,
        evals: &[Scalar],
        gens: &SparseMatPolyCommitmentGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        comm: &SparseMatPolyCommitment,
        comm_derefs: &DerefsCommitment,
        evals: &[Scalar],
        gens: &SparseMatPolyCommitmentGens,
        rx: &[Scalar],
        ry: &[Scalar],
        r_mem_check: &(Scalar, Scalar),
        nz: usize,
        transcript: &mut Transcript,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SparseMatPolyEvalProof {
    comm_derefs: DerefsCommitment,
    poly_eval_network_proof: PolyEvalNetworkProof,
}
impl SparseMatPolyEvalProof {
    fn protocol_name() -> &'static [u8] {
        panic!("STUB: not implemented");
    }
    fn equalize(rx: &[Scalar], ry: &[Scalar]) -> (Vec<Scalar>, Vec<Scalar>) {
        panic!("STUB: not implemented");
    }
    pub fn prove(
        dense: &MultiSparseMatPolynomialAsDense,
        rx: &[Scalar],
        ry: &[Scalar],
        evals: &[Scalar],
        gens: &SparseMatPolyCommitmentGens,
        transcript: &mut Transcript,
        random_tape: &mut RandomTape,
    ) -> SparseMatPolyEvalProof {
        panic!("STUB: not implemented");
    }
    pub fn verify(
        &self,
        comm: &SparseMatPolyCommitment,
        rx: &[Scalar],
        ry: &[Scalar],
        evals: &[Scalar],
        gens: &SparseMatPolyCommitmentGens,
        transcript: &mut Transcript,
    ) -> Result<(), ProofVerifyError> {
        panic!("STUB: not implemented");
    }
}
pub struct SparsePolyEntry {
    idx: usize,
    val: Scalar,
}
impl SparsePolyEntry {
    pub fn new(idx: usize, val: Scalar) -> Self {
        panic!("STUB: not implemented");
    }
}
pub struct SparsePolynomial {
    num_vars: usize,
    Z: Vec<SparsePolyEntry>,
}
impl SparsePolynomial {
    pub fn new(num_vars: usize, Z: Vec<SparsePolyEntry>) -> Self {
        panic!("STUB: not implemented");
    }
    fn compute_chi(a: &[bool], r: &[Scalar]) -> Scalar {
        panic!("STUB: not implemented");
    }
    pub fn evaluate(&self, r: &[Scalar]) -> Scalar {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    use rand::RngCore;
    #[test]
    fn check_sparse_polyeval_proof() {
        let mut csprng: OsRng = OsRng;
        let num_nz_entries: usize = 256;
        let num_rows: usize = 256;
        let num_cols: usize = 256;
        let num_vars_x: usize = num_rows.log_2();
        let num_vars_y: usize = num_cols.log_2();
        let M = (0..num_nz_entries)
            .map(|_i| {
                SparseMatEntry::new(
                    (csprng.next_u64() % (num_rows as u64)) as usize,
                    (csprng.next_u64() % (num_cols as u64)) as usize,
                    Scalar::random(&mut csprng),
                )
            })
            .collect();
        let poly_M = SparseMatPolynomial::new(num_vars_x, num_vars_y, M);
        let gens = SparseMatPolyCommitmentGens::new(
            b"gens_sparse_poly",
            num_vars_x,
            num_vars_y,
            num_nz_entries,
            3,
        );
        let (poly_comm, dense) = SparseMatPolynomial::multi_commit(
            &[&poly_M, &poly_M, &poly_M],
            &gens,
        );
        let rx: Vec<Scalar> = (0..num_vars_x)
            .map(|_i| Scalar::random(&mut csprng))
            .collect::<Vec<Scalar>>();
        let ry: Vec<Scalar> = (0..num_vars_y)
            .map(|_i| Scalar::random(&mut csprng))
            .collect::<Vec<Scalar>>();
        let eval = SparseMatPolynomial::multi_evaluate(&[&poly_M], &rx, &ry);
        let evals = vec![eval[0], eval[0], eval[0]];
        let mut random_tape = RandomTape::new(b"proof");
        let mut prover_transcript = Transcript::new(b"example");
        let proof = SparseMatPolyEvalProof::prove(
            &dense,
            &rx,
            &ry,
            &evals,
            &gens,
            &mut prover_transcript,
            &mut random_tape,
        );
        let mut verifier_transcript = Transcript::new(b"example");
        assert!(
            proof.verify(& poly_comm, & rx, & ry, & evals, & gens, & mut
            verifier_transcript,).is_ok()
        );
    }
}
