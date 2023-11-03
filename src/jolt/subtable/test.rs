#[macro_export]
macro_rules! subtable_materialize_mle_parity_test {
  ($test_name:ident, $subtable_type:ty, $F:ty, $M:expr) => {
    #[test]
    fn $test_name() {
      const M: usize = $M;
      let log_M = ark_std::log2(M) as usize;

      let subtable = <$subtable_type>::new();
      let materialized: Vec<_> = subtable.materialize(M);
      for i in 0..M {
        assert_eq!(
          materialized[i],
          subtable.evaluate_mle(&crate::utils::index_to_field_bitvector(i, log_M)),
          "MLE did not match materialized subtable at index {}",
          i
        );
      }
    }
  };
}
