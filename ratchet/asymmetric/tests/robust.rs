#![feature(macro_metavar_expr)]

use rand_core::OsRng;

use k256::ProjectivePoint;

use asymmetric_ratchet::{AsymmetricRatchet, DiffieHellman, robust_ratchet};

robust_ratchet!(DoubleHellman, DiffieHellman::<ProjectivePoint>, DiffieHellman::<ProjectivePoint>);

#[test]
fn robust() {
  let mut a = DoubleHellman::default();
  let a_key = a.step(&mut OsRng);

  let mut b = DoubleHellman::default();
  let b_key = b.step(&mut OsRng);

  assert_eq!(a.handshake(b_key), b.handshake(a_key));
}
