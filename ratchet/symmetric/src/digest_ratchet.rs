use zeroize::Zeroize;

use digest::{Digest, Output, FixedOutputReset};

use crate::SymmetricRatchet;

pub struct DigestRatchet<D: Digest + FixedOutputReset> {
  constant: &'static [u8],
  key: Output<D>,
}

impl<D: Digest + FixedOutputReset> Zeroize for DigestRatchet<D> {
  fn zeroize(&mut self) {
    for b in self.key.iter_mut() {
      *b = 0;
    }
  }
}

impl<D: Digest + FixedOutputReset> Drop for DigestRatchet<D> {
  fn drop(&mut self) {
    self.zeroize()
  }
}

impl<D: Digest + FixedOutputReset> DigestRatchet<D> {
  fn hash(&self, dst: &'static [u8]) -> Output<D> {
    D::new()
      .chain_update(self.constant)
      .chain_update(dst)
      .chain_update(self.key.as_ref())
      .finalize_reset()
  }
}

impl<D: Digest + FixedOutputReset> SymmetricRatchet for DigestRatchet<D> {
  type Output = Output<D>;

  fn new(dst: &'static [u8], key: Output<D>) -> DigestRatchet<D> {
    DigestRatchet { constant: dst, key }
  }

  fn chain_key(self) -> Output<D> {
    self.key.clone()
  }

  fn step(&mut self) -> Output<D> {
    let res = self.hash(b"message");
    self.key = self.hash(b"chain_key");
    res
  }
}
