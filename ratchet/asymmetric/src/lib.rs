#![cfg_attr(feature = "std", feature(macro_metavar_expr))]

use zeroize::{Zeroize, ZeroizeOnDrop};
use rand_core::{RngCore, CryptoRng};

#[cfg(feature = "std")]
mod robust;

#[cfg(feature = "diffie_hellman")]
pub mod diffie_hellman;

pub trait AsymmetricRatchet: Zeroize + ZeroizeOnDrop {
  type PublicKey: Default + AsRef<[u8]> + AsMut<[u8]>;
  type Output: Zeroize + AsRef<[u8]>;

  fn step<R: RngCore + CryptoRng>(&mut self, rng: &mut R) -> Self::PublicKey;

  fn handshake(&self, key: Self::PublicKey) -> Option<Self::Output>;
}
