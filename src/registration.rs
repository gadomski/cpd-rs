use {Matrix, Normalization, UInt};
use gauss_transform::Probabilities;
use generic_array::ArrayLength;
use nalgebra::DimName;
use std::ops::Mul;

/// A trait for all structures that can be registered by a runner.
pub trait Registration<D>
where
    D: DimName,
    <D as DimName>::Value: Mul + Mul<UInt>,
    <<D as DimName>::Value as Mul<UInt>>::Output: ArrayLength<f64>,
{
    /// The struct that is returned after a registration. Holds the registration information, e.g.
    /// rotation and translation matrices.
    type Transform;

    /// Returns a reference to this registration moved-points matrix.
    ///
    /// # Examples
    ///
    /// Rigid's registration implements `Registration`:
    ///
    /// ```
    /// use cpd::Rigid;
    /// let rigid = Rigid::new();
    /// let moved = rigid.moved();
    /// ```
    fn moved(&self) -> &Matrix<D>;

    /// Perform one iteration of the registration.
    ///
    /// # Examples
    ///
    /// Rigid's registration implements `Registration`:
    ///
    /// ```
    /// use cpd::{Rigid, utils, Probabilities};
    /// let rigid = Rigid::new();
    /// let registration = rigid.as_registration().unwrap();
    /// let matrix = utils::random_matrix2(10);
    /// let probabilities = Probabilities::new(&matrix, &matrix, 1.0, 0.1);
    /// let sigma2 = registration.iterate(&matrix, &matrix, &probabilities);
    /// ```
    fn iterate(
        &mut self,
        fixed: &Matrix<D>,
        moving: &Matrix<D>,
        probabilities: &Probabilities<D>,
    ) -> f64;

    /// Denormalize the registration.
    ///
    /// # Examples
    ///
    /// Rigid's registration implements 'Registration':
    ///
    /// ```
    /// use cpd::Rigid;
    /// let rigid = Rigid::new();
    /// let registration = rigid.as_registration().unwrap();
    /// let normalization = Normalization::default();
    /// registration.denormalize(&normalization);
    /// ```
    fn denormalize(&mut self, normalization: &Normalization<D>);
}
