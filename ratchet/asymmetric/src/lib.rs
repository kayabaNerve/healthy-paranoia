#[cfg(feature = "diffie_helman")]
pub mod diffie_helman;

pub trait AsymmetricRatchet {
  type PublicKey;
  type Output;

  fn step<R: rand::CryptoRng + rand::RngCore>(&mut self, rng: R) -> Self::PublicKey;

  fn handshake(&self, key: Self::PublicKey) -> Self::Output;
}
