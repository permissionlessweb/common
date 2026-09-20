//! Polynomial payload helpers for circuit-key encodings.

use crate::poly::{EvaluationDomain, LagrangeCoeff, Polynomial};
use ff::{PrimeField, WithSmallOrderMulGroup};
use std::io;

pub(super) fn write_polynomial<W: io::Write, F: PrimeField>(
    writer: &mut W,
    values: &[F],
) -> io::Result<()> {
    let len = u32::try_from(values.len()).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "polynomial length exceeds u32")
    })?;
    writer.write_all(&len.to_le_bytes())?;
    for value in values {
        writer.write_all(value.to_repr().as_ref())?;
    }
    Ok(())
}

pub(super) fn read_polynomial<R: io::Read, F: WithSmallOrderMulGroup<3>>(
    reader: &mut R,
    domain: &EvaluationDomain<F>,
) -> io::Result<Polynomial<F, LagrangeCoeff>> {
    let mut len = [0; 4];
    reader.read_exact(&mut len)?;
    let len = u32::from_le_bytes(len) as usize;
    if len != 1usize << domain.k() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unexpected polynomial length",
        ));
    }
    let mut values = Vec::new();
    for _ in 0..len {
        let mut repr = F::Repr::default();
        reader.read_exact(repr.as_mut())?;
        values.push(Option::<F>::from(F::from_repr(repr)).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "invalid field encoding in key")
        })?);
    }
    Ok(domain.lagrange_from_vec(values))
}
