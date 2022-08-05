use rand_core::{CryptoRng, RngCore};

use group::{ff::Field, prime::PrimeGroup};

use crate::AsymmetricRatchet;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct DiffieHellman<G: PrimeGroup>(G::Scalar);

impl<G: PrimeGroup> AsymmetricRatchet for DiffieHellman<G> where G::Scalar: Zeroize {
  type PublicKey = G;
  type Output = G::Repr;

  fn step<R: RngCore + CryptoRng>(&mut self, rng: &mut R) -> Self::PublicKey {
    self.0 = G::Scalar::random(rng);
    G::generator() * self.0
  }

  fn handshake(&self, key: Self::PublicKey) -> Self::Output {
    (key * self.0).to_bytes()
  }
}
