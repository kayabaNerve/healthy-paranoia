#[cfg(feature = "digest_ratchet")]
mod digest_ratchet;
#[cfg(feature = "digest_ratchet")]
pub use digest_ratchet::DigestRatchet;

/// A Symmetric Ratchet which can be advanced as needed.
trait SymmetricRatchet {
  type Output: AsRef<[u8]>;

  /// Create a new ratchet using a constant DST and an initial chain key, either one newly
  /// generated or the ratchet's previous chain key which is now being restored.
  fn new(dst: &'static [u8], initial: Self::Output) -> Self;

  /// Retrieve the ratchet's chain key.
  fn chain_key(self) -> Self::Output;

  /// Step the ratchet, returning a key distinct from the ratchet's chain key.
  fn step(&mut self) -> Self::Output;
}
