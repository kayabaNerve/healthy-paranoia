use zeroize::{Zeroize, ZeroizeOnDrop};
use rand_core::{CryptoRng, RngCore};

use group::{ff::Field, prime::PrimeGroup};

use crate::AsymmetricRatchet;

pub struct DiffieHellman<G: PrimeGroup>(G::Scalar) where G::Scalar: Zeroize;
impl<G: PrimeGroup> Zeroize for DiffieHellman<G> where G::Scalar: Zeroize {
  fn zeroize(&mut self) {
    self.0.zeroize();
  }
}

impl<G: PrimeGroup> Drop for DiffieHellman<G> where G::Scalar: Zeroize {
  fn drop(&mut self) {
    self.zeroize();
  }
}

impl<G: PrimeGroup> ZeroizeOnDrop for DiffieHellman<G> where G::Scalar: Zeroize {}

impl<G: PrimeGroup> AsymmetricRatchet for DiffieHellman<G> where G::Repr: Zeroize, G::Scalar: Zeroize {
  type PublicKey = G::Repr;
  type Output = G::Repr;

  fn step<R: RngCore + CryptoRng>(&mut self, rng: &mut R) -> Self::PublicKey {
    self.0 = G::Scalar::random(rng);
    (G::generator() * self.0).to_bytes()
  }

  fn handshake(&self, key: Self::PublicKey) -> Option<Self::Output> {
    let key = G::from_bytes(&key);
    if key.is_none().into() {
      return None;
    }
    Some((key.unwrap() * self.0).to_bytes())
  }
}
