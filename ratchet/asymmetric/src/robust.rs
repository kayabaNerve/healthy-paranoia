#[macro_export]
macro_rules! robust_ratchet {
  ($vis: vis $name: ident, $($Ratchet: ty),+) => {
    $vis struct $name($($Ratchet),*);

    impl zeroize::Zeroize for $name {
      fn zeroize(&mut self) {
        $(
          let _: $Ratchet;
          self.${index()}.zeroize();
        )*
      }
    }

    impl Drop for $name {
      fn drop(&mut self) {
        use zeroize::Zeroize;
        self.zeroize()
      }
    }

    impl zeroize::ZeroizeOnDrop for $name {}

    impl Default for $name {
      fn default() -> $name {
        $name(
          $(<$Ratchet>::default()),*
        )
      }
    }

    impl asymmetric_ratchet::AsymmetricRatchet for $name {
      type PublicKey = Vec<u8>;
      type Output = Vec<u8>;

      fn step<
        R: rand_core::RngCore + rand_core::CryptoRng
      >(&mut self, rng: &mut R) -> Self::PublicKey {
        let mut res = Vec::<u8>::new();
        $(
          let key: <
            $Ratchet as asymmetric_ratchet::AsymmetricRatchet
          >::PublicKey = self.${index()}.step(rng);
          let key_ref: &[u8] = key.as_ref();
          res.extend(key_ref);
        )*
        res
      }

      fn handshake(&self, key: Self::PublicKey) -> Option<Self::Output> {
        use std::io::Read;
        use zeroize::Zeroize;

        let mut cursor = std::io::Cursor::new(key);
        let mut res = Vec::<u8>::new();

        $({
          let mut buffer = <
            $Ratchet as asymmetric_ratchet::AsymmetricRatchet
          >::PublicKey::default();
          cursor.read_exact(buffer.as_mut()).ok()?;

          let mut raw = self.${index()}.handshake(buffer)?;
          let raw_ref: &[u8] = raw.as_ref();
          res.extend(raw_ref);
          raw.zeroize();
        })*

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
