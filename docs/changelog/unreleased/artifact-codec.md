## zakura-halo2-proofs

### Added

- Added little-endian `ConstraintSystem::{write,read}` and
  `VerifyingKey::{write,read_with_cs}` so hosts can store circuit artifacts
  without calling `Circuit::configure` and without exposing verifying-key
  fields.
