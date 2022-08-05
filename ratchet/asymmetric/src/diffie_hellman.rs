use rand::{CryptoRng, RngCore};
use group::{ff::Field, prime::PrimeGroup};

use crate::AsymmetricRatchet;

pub struct DiffieHellman<G: PrimeGroup> {
  key: G::Scalar,
}

impl<G: PrimeGroup> AsymmetricRatchet for DiffieHellman<G> {
  type PublicKey = G;
  type Output = G::Repr;

  fn step<R: RngCore + CryptoRng>(&mut self, rng: R) -> Self::PublicKey {
    self.key = G::Scalar::random(rng);
    G::generator() * self.key
  }

  fn handshake(&self, key: Self::PublicKey) -> Self::Output {
    (key * self.key).to_bytes()
  }
}
