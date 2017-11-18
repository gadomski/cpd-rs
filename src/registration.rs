use {Iteration, Matrix};
use gauss_transform::Probabilities;
use nalgebra::DimName;

/// A trait for all structures that can be registered by a runner.
pub trait Registration<D>
where
    D: DimName,
{
    /// The struct that is returned after a registration. Holds the registration information, e.g.
    /// rotation and translation matrices.
    type Transform;

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
    /// let iteration = registration.iterate(&matrix, &matrix, &probabilities);
    /// ```
    fn iterate(
        &mut self,
        fixed: &Matrix<D>,
        moving: &Matrix<D>,
        probabilities: &Probabilities<D>,
    ) -> Iteration<D>;
}
