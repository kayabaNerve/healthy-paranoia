use crate::AsymmetricRatchet;
use rand;
use group::prime::PrimeGroup;
use group::ff::Field;

pub struct Ratchet<P: PrimeGroup> {
  key: P::Scalar,
}

impl<G: PrimeGroup> AsymmetricRatchet for Ratchet<G> {
  type PublicKey = G;

  fn step<R: rand::CryptoRng + rand::RngCore>(&mut self, rng: R) -> Self::PublicKey {
    self.key = G::Scalar::random(rng);
    G::generator() * self.key
  }

  fn handshake(&self, key: Self::PublicKey) -> Self::PublicKey {
    key * self.key
  }
}
