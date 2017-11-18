use Matrix;
use gauss_transform::{InvalidOutlierWeight, Probabilities};
use nalgebra::DimName;

/// Runs gauss transforms on two point sets.
#[derive(Debug)]
pub struct Transformer<'a, D>
where
    D: DimName,
{
    fixed: &'a Matrix<D>,
    outlier_weight: f64,
}

impl<'a, D> Transformer<'a, D>
where
    D: DimName,
{
    /// Creates a new transformer.
    ///
    /// Returns an error if the outlier weight is not between zero and one.
    ///
    /// # Examples
    ///
    /// ```
    /// use cpd::utils;
    /// use cpd::gauss_transform::Transformer;
    /// let matrix = utils::random_matrix2(10);
    /// let transformer = Transformer::new(&matrix, 0.1).unwrap();
    /// assert!(Transformer::new(&matrix, 1.1).is_err());
    /// ```
    pub fn new(
        fixed: &'a Matrix<D>,
        outlier_weight: f64,
    ) -> Result<Transformer<'a, D>, InvalidOutlierWeight> {
        if outlier_weight < 0. || outlier_weight > 1. {
            Err(InvalidOutlierWeight(outlier_weight))
        } else {
            Ok(Transformer {
                fixed: fixed,
                outlier_weight: outlier_weight,
            })
        }
    }

    /// Returns probabilities as calculated for these moving points and sigma2.
    ///
    /// # Examples
    ///
    /// ```
    /// use cpd::utils;
    /// use cpd::gauss_transform::Transformer;
    /// let fixed = utils::random_matrix2(10);
    /// let transformer = Transformer::new(&fixed, 0.1).unwrap();
    /// let moving = utils::random_matrix2(10);
    /// let probabilities = transformer.probabilities(&moving, 1.0);
    /// ```
    pub fn probabilities(&self, _moving: &Matrix<D>, _sigma2: f64) -> Probabilities<D> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils;

    #[test]
    fn invalid_outlier_weight() {
        let matrix = utils::random_matrix2(10);
        assert_eq!(
            InvalidOutlierWeight(-1.),
            Transformer::new(&matrix, -1.).unwrap_err()
        );
        assert_eq!(
            InvalidOutlierWeight(1.1),
            Transformer::new(&matrix, 1.1).unwrap_err()
        );
    }
}
