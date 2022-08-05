use zeroize::{Zeroize, ZeroizeOnDrop};
use rand_core::{RngCore, CryptoRng};

#[cfg(feature = "diffie_helman")]
pub mod diffie_helman;

pub trait AsymmetricRatchet: Zeroize + ZeroizeOnDrop {
  type PublicKey: Default + AsRef<[u8]> + AsMut<[u8]>;
  type Output: Zeroize + AsRef<[u8]>;

  fn step<R: RngCore + CryptoRng>(&mut self, rng: &mut R) -> Self::PublicKey;

  fn handshake(&self, key: Self::PublicKey) -> Self::Output;
}
