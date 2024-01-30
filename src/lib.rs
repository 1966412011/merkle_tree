pub mod merkle;
use ff::Field;
use ff::*;
use poseidon_rs::{Fr, Poseidon};

use sha2::Digest;

fn sha256_hash(msg: &[u8]) -> [u8; 32] {
    let mut hasher = sha2::Sha256::new();
    hasher.update(msg);
    hasher.finalize().into()
}

pub fn gen_poseidon_hash(msg: &[u8]) -> anyhow::Result<Fr> {
    let sha256 = sha256_hash(msg);
    let mut fr = Fr::zero();
    let n256 =
        Fr::from_str("256").ok_or_else(|| anyhow::anyhow!("gen_poseidon_hash parse error 1"))?;
    for v in sha256 {
        fr.mul_assign(&n256);
        fr.add_assign(
            &Fr::from_str(&v.to_string())
                .ok_or_else(|| anyhow::anyhow!("gen_poseidon_hash parse error 2"))?,
        );
    }
    let big_arr = vec![fr];
    gen_poseidon_hash_by_fr(big_arr)
}

pub fn gen_poseidon_hash_by_fr(big_arr: Vec<Fr>) -> anyhow::Result<Fr> {
    let poseidon = Poseidon::new();
    poseidon
        .hash(big_arr)
        .map_err(|e| anyhow::anyhow!("gen_poseidon_hash poseidon hash error {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = gen_poseidon_hash(b"hello").unwrap();
        let b = gen_poseidon_hash(b"world").unwrap();
        let c = gen_poseidon_hash(b"hello world").unwrap();
        println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);
        let ab = gen_poseidon_hash_by_fr(vec![a, b]).unwrap();
        let bc = gen_poseidon_hash_by_fr(vec![b, c]).unwrap();
        println!("ab: {:?}, bc: {:?}", ab, bc);
        let abc = gen_poseidon_hash_by_fr(vec![ab, c]).unwrap();
        let abc_ = gen_poseidon_hash_by_fr(vec![a, bc]).unwrap();
        println!("abc: {:?}, abc_: {:?}", abc, abc_);
    }
}
