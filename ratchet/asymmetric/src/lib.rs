pub mod asymmetric_ratchet;

pub trait AsymmetricRatchet {
  type PublicKey;

  fn step<R: rand::CryptoRng + rand::RngCore>(&mut self, rng: R) -> Self::PublicKey;

  fn handshake(&self, key: Self::PublicKey) -> Self::PublicKey;
}
