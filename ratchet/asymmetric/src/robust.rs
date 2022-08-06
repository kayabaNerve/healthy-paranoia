use std::io::Read;

use zeroize::Zeroize;
use rand_core::{RngCore, CryptoRng};

use crate::AsymmetricRatchet;

#[allow(unused)]
fn vec_step<R: RngCore + CryptoRng, T: AsymmetricRatchet>(ratchet: &mut T, rng: &mut R) -> Vec<u8> {
  ratchet.step(rng).as_ref().to_vec()
}

#[allow(unused)]
fn vec_handshake<R: Read, T: AsymmetricRatchet>(
  ratchet: &T,
  cursor: &mut R,
  res: &mut Vec<u8>,
) -> Option<()> {
  let mut buffer = T::PublicKey::default();
  cursor.read_exact(buffer.as_mut()).ok()?;
  let mut raw = ratchet.handshake(buffer)?;
  res.extend(raw.as_ref());
  raw.zeroize();
  Some(())
}

#[macro_export]
macro_rules! robust_ratchet {
  ($vis: vis $name: ident, $($ratchet: ty),+) => {
    $vis struct $name($($ratchet),*);

    impl Zeroize for $name {
      fn zeroize(&mut self) {
        $(
          let _: $ratchet;
          self.${index()}.zeroize();
        )*
      }
    }

    impl Drop for $name {
      fn drop(&mut self) {
        self.zeroize()
      }
    }

    impl zeroize::ZeroizeOnDrop for $name {}

    impl AsymmetricRatchet for $name {
      type PublicKey = Vec<u8>;
      type Output = Vec<u8>;

      fn step<R: RngCore + CryptoRng>(&mut self, rng: &mut R) -> Self::PublicKey {
        let mut res = vec![];
        $(
          let _: $ratchet;
          res.extend(vec_step(&mut self.${index()}, rng));
        )*
        res
      }

      fn handshake(&self, key: Self::PublicKey) -> Option<Self::Output> {
        let mut cursor = std::io::Cursor::new(key);
        let mut res = vec![];
        $(
          let _: $ratchet;
          vec_handshake(&self.${index()}, &mut cursor, &mut res)?;
        )*
        Some(res)
      }
    }
  }
}

// Work on a proc_macro variant of the above.
// If done properly, it'd remove the reliance on nightly.
/*
#[proc_macro]
pub fn robust_ratchet(ratchets: TokenStream) -> TokenStream {
  let ratchets = ratchets.iter();
  let name = ratchet.next();

  let mut tuple;     // A, B
  let mut step;      // res.extend(vec_step(self.0)); res.extend(vec_step(self.1));
  let mut handshake; // vec_handshake(&self.0, &mut cursor, &mut res);
                     // vec_handshake(&self.1, &mut cursor, &mut res);
  let mut zeroize;   // self.0.zeroize(); self.1.zeroize();

  for ratchet in ratchets {

  }

  quote! {
    struct #name(#tuple);

    impl Zeroize for #name {
      fn zeroize(&mut self) {
        #zeroize
      }
    }

    impl Drop for #name {
      fn drop(&mut self) {
        self.zeroize()
      }
    }

    impl ZeroizeOnDrop for RobustRatchet {}

    impl AsymmetricRatchet for RobustRatchet {
      type PublicKey = Vec<u8>;
      type Output = Vec<u8>;

      fn step<R: RngCore + CryptoRng>(&mut self, rng: &mut R) -> Self::PublicKey {
        let mut res = vec![];
        #step
        res
      }

      fn handshake(&self, key: Self::PublicKey) -> Self::Output {
        let mut cursor = Cursor::new(key);
        let mut res = vec![];
        #handshake
        res
      }
    }
  }
}
*/
