//! Registers two point sets with Coherent Point Drift (cpd).
//!
//! Coherent Point Drifit is a point set registration algorithm created by [Andriy
//! Myroneno](https://sites.google.com/site/myronenko/research/cpd). It calculates the best
//! alignment between two point sets using one of three algorithms. Some of these algorithms are
//! currently implemented in **cpd-rs**, and some are not. This is a pure-rust implementation of
//! cpd, relying on [nalgebra](http://nalgebra.org/) for the linear algebra.
//!
//!
//! ## Implemented
//!
//! - **rigid**: rotation, translation, and optional scaling
//!
//! ## Not implemented
//!
//! - **nonrigid**: nonrigid transformation goverend by motion
//! coherence theory.
//! - **affine**: an affine matrix transformation.
//!
//! # Architecture
//!
//! Some parts of a cpd run, e.g. the maximum number of iterations or the outlier weight, are
//! common among all flavors of cpd. Use a `Runner` to configure these common variables:
//!
//! ```
//! let runner = cpd::Runner::new().max_iterations(10).outlier_weight(0.2);
//! ```
//!
//! A `Runner` can be converted to a `Rigid` (for example), which is itself a builder:
//!
//! ```
//! let rigid = cpd::Runner::new().rigid().allow_reflections(false);
//! ```
//!
//! `Rigid` can be used to run the registration on two matrices:
//!
//! ```
//! use cpd::{Rigid, utils};
//! let fixed = utils::random_matrix2(10);
//! let moving = utils::random_matrix2(10);
//! let rigid = Rigid::new();
//! let run = rigid.register(&fixed, &moving).unwrap();
//! ```

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]

extern crate alga;
#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate generic_array;
#[cfg(feature = "las")]
extern crate las;
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
pub type UInt =
    generic_array::typenum::UInt<generic_array::typenum::UTerm, generic_array::typenum::B1>;
