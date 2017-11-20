//! Register two point sets with Coherent Point Drift (cpd).
//!
//! Use `Rigid` to run rigid cpd:
//!
//! ```
//! use cpd::{Rigid, utils};
//! let rigid = Rigid::new();
//! let fixed = utils::random_matrix2(10);
//! let moving = utils::random_matrix2(10);
//! let (transform, run) = rigid.register(&fixed, &moving).unwrap();
//! assert!(run.converged);
//! println!("{}", transform.rotation);
//! println!("{}", transform.translation);
//! ```
//!
//! Configure cpd runs with a `Runner`, and configure rigid-specific parameters with `Rigid`:
//!
//! ```
//! use cpd::Runner;
//! let rigid = Runner::new()
//!     .max_iterations(100)
//!     .rigid()
//!     .scale(true);
//! ```

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate generic_array;
#[macro_use]
extern crate log;
extern crate nalgebra;

pub mod gauss_transform;
pub mod normalize;
pub mod rigid;
pub mod runner;
pub mod utils;
mod registration;

pub use nalgebra::{U2, U3};
pub use normalize::{Normalization, Normalize};
pub use registration::Registration;
pub use rigid::Rigid;
pub use runner::{Run, Runner};

/// Our custom dynamic-row matrix type.
pub type Matrix<D> = nalgebra::MatrixMN<f64, nalgebra::Dynamic, D>;

/// Our custom square matrix type.
pub type SquareMatrix<D> = nalgebra::MatrixN<f64, D>;

/// Our custom vector type.
pub type Vector<D> = nalgebra::VectorN<f64, D>;

/// Our custom row vector type.
pub type RowVector<D> = nalgebra::RowVectorN<f64, D>;

/// Our UInt, used for matrix indexing.
pub type UInt = generic_array::typenum::UInt<
    generic_array::typenum::UTerm,
    generic_array::typenum::B1,
>;
