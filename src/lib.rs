#![feature(zero_one)]

extern crate libc;

use ffi::*;

use libc::*;
//use std::cell::{RefCell};
use std::fmt::{Debug};
use std::mem::{transmute};
use std::num::{Zero};
//use std::rc::{Rc};

pub mod ffi;

#[derive(Clone, Copy, Debug)]
pub enum QuadpackError {
  MaxSubdivisions,
  RoundoffError,
  BadIntegrand,
  NoConverge,
  Divergent,
  InvalidInput,
}

pub struct QuadpackScratch<T> where T: Copy {
  iwork:    Vec<i32>,
  work:     Vec<T>,
}

impl<T> QuadpackScratch<T> where T: Copy + Zero {
  pub fn new(len: usize) -> QuadpackScratch<T> {
    let mut iwork = Vec::with_capacity(len);
    for _ in 0 .. len {
      iwork.push(0);
    }
    let mut work = Vec::with_capacity(len * 4);
    for _ in 0 .. len * 4 {
      work.push(T::zero());
    }
    QuadpackScratch{
      iwork:    iwork,
      work:     work,
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct QuadpackResult<T> where T: Copy + Debug {
  pub value:    T,
  pub abserr:   T,
  pub neval:    usize,
  pub nsubdiv:  usize,
}

pub struct QuadpackIntegrand<T, Data> where T: Copy {
  pub f:    extern "C" fn (x: *mut T) -> T,
  //pub f:    extern "C" fn (x: *mut T, data: *mut c_void),
  pub data: Data,
}

impl<Data> QuadpackIntegrand<f64, Data> {
  pub fn integrate_qags(&mut self, mut a: f64, mut b: f64, mut epsabs: f64, mut epsrel: f64, scratch: &mut QuadpackScratch<f64>) -> Result<QuadpackResult<f64>, QuadpackError> {
    let data: *mut c_void = unsafe { transmute(&mut self.data) };
    let mut result: f64 = 0.0;
    let mut abserr: f64 = 0.0;
    let mut neval: c_int = 0;
    let mut ier: c_int = 0;
    let mut limit: c_int = scratch.iwork.len() as _;
    let mut lenw: c_int = scratch.work.len() as _;
    let mut last: c_int = 0;
    assert!(lenw >= 4 * limit);
    unsafe { dqags_(
        self.f,
        &mut a as *mut _,
        &mut b as *mut _,
        &mut epsabs as *mut _,
        &mut epsrel as *mut _,
        &mut result as *mut _,
        &mut abserr as *mut _,
        &mut neval as *mut _,
        &mut ier as *mut _,
        &mut limit as *mut _,
        &mut lenw as *mut _,
        &mut last as *mut _,
        scratch.iwork.as_mut_ptr(),
        scratch.work.as_mut_ptr(),
    ) };
    match ier {
      0 => Ok(QuadpackResult{
        value:      result,
        abserr:     abserr,
        neval:      neval as _,
        nsubdiv:    last as _,
      }),
      1 => Err(QuadpackError::MaxSubdivisions),
      2 => Err(QuadpackError::RoundoffError),
      3 => Err(QuadpackError::BadIntegrand),
      4 => Err(QuadpackError::NoConverge),
      5 => Err(QuadpackError::Divergent),
      6 => Err(QuadpackError::InvalidInput),
      _ => unreachable!(),
    }
  }
}

#[test]
fn test_qags() {
  let epsrel = 1.0e-6;
  extern "C" fn integrand_f(x: *mut f64) -> f64 {
    3.14
  }
  let mut integrand = QuadpackIntegrand{
    f:    integrand_f,
    data: (),
  };
  let mut scratch = QuadpackScratch::new(1024);
  let res = integrand.integrate_qags(0.0, 1.0, 0.0, epsrel, &mut scratch);
  //panic!("integral: {:?}", res);
  assert!(((res.unwrap().value - 3.14) / 3.14).abs() < 2.0 * epsrel);
}
