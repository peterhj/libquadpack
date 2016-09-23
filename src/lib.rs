#![feature(zero_one)]

extern crate libc;

pub use self::QuadpackInterval::*;

use ffi::*;

use libc::*;
//use std::cell::{RefCell};
use std::fmt::{Debug};
use std::mem::{size_of, transmute};
use std::num::{Zero};
//use std::ptr::{null_mut};
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
  pub fn new(sz_bytes: usize) -> QuadpackScratch<T> {
    let quantum = (size_of::<i32>() + 4 * size_of::<T>());
    let max_len = (sz_bytes + quantum - 1) / quantum;
    //println!("scratch: {}", max_len);
    assert!(max_len >= 1);
    QuadpackScratch::with_max_subdivs(max_len)
  }

  pub fn with_max_subdivs(len: usize) -> QuadpackScratch<T> {
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

#[derive(Clone, Copy, Debug)]
pub enum QuadpackInterval<T> {
  Bounded(T, T),
  UpperInf(T),
  LowerInf(T),
  Infinite,
}

pub struct QuadpackIntegrand<T, U> where T: Copy {
  pub f:    extern "C" fn (x: *mut T, udata: *mut c_void) -> T,
  pub data: U,
}

impl<U> QuadpackIntegrand<f32, U> {
  pub fn integrate_qags(&mut self, mut a: f32, mut b: f32, mut epsabs: f32, mut epsrel: f32, scratch: &mut QuadpackScratch<f32>) -> Result<QuadpackResult<f32>, QuadpackError> {
    let udata: *mut c_void = unsafe { transmute(&mut self.data) };
    let mut result: f32 = 0.0;
    let mut abserr: f32 = 0.0;
    let mut neval: c_int = 0;
    let mut ier: c_int = 0;
    let mut limit: c_int = scratch.iwork.len() as _;
    let mut lenw: c_int = scratch.work.len() as _;
    let mut last: c_int = 0;
    assert!(lenw >= 4 * limit);
    unsafe { qags_(
        self.f,
        udata,
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

impl<U> QuadpackIntegrand<f64, U> {
  /*pub fn integrate_qags(&mut self, mut a: f64, mut b: f64, mut epsabs: f64, mut epsrel: f64, scratch: &mut QuadpackScratch<f64>) -> Result<QuadpackResult<f64>, QuadpackError> {
    let udata: *mut c_void = unsafe { transmute(&mut self.data) };
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
        udata,
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
  }*/

  /*pub fn integrate_qagi(&mut self, interval: QuadpackInterval<f64>, mut epsabs: f64, mut epsrel: f64, scratch: &mut QuadpackScratch<f64>) -> Result<QuadpackResult<f64>, QuadpackError> {
    let udata: *mut c_void = unsafe { transmute(&mut self.data) };
    /*let (mut bound, mut inf): (f64, c_int) = match interval {
      Bounded(_, _) => unimplemented!(),
      UpperInf(lo)  => (lo, 1),
      LowerInf(hi)  => (hi, -1),
      Infinite      => (0.0, 2),
    };*/
    let mut bound: f64 = 0.0;
    let mut inf: c_int = 2;
    let mut result: f64 = 0.0;
    let mut abserr: f64 = 0.0;
    let mut neval: c_int = 0;
    let mut ier: c_int = 0;
    let mut limit: c_int = scratch.iwork.len() as _;
    let mut lenw: c_int = scratch.work.len() as _;
    let mut last: c_int = 0;
    assert!(lenw >= 4 * limit);
    unsafe { dqagi_(
        self.f,
        udata,
        &mut bound as *mut _,
        &mut inf as *mut _,
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
  }*/

  pub fn integrate(&mut self, mut interval: QuadpackInterval<f64>, mut epsabs: f64, mut epsrel: f64, scratch: &mut QuadpackScratch<f64>) -> Result<QuadpackResult<f64>, QuadpackError> {
    let mut result: f64 = 0.0;
    let mut abserr: f64 = 0.0;
    let mut neval: c_int = 0;
    let mut ier: c_int = 0;
    let mut limit: c_int = scratch.iwork.len() as _;
    let mut lenw: c_int = scratch.work.len() as _;
    let mut last: c_int = 0;
    assert!(lenw >= 4 * limit);
    let udata: *mut c_void = unsafe { transmute(&mut self.data) };
    match interval {
      Bounded(mut a, mut b) => {
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
            udata,
        ) };
      }
      _ => {
        let (mut bound, mut inf): (f64, c_int) = match interval {
          Bounded(_, _) => unimplemented!(),
          UpperInf(lo)  => (lo, 1),
          LowerInf(hi)  => (hi, -1),
          Infinite      => (0.0, 2),
        };
        unsafe { dqagi_(
            self.f,
            &mut bound as *mut _,
            &mut inf as *mut _,
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
            udata,
        ) };
      }
    };
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
fn test_qags_qagi() {
  {
    let epsrel = 1.0e-5;
    extern "C" fn integrand_f32(x: *mut f32, udata: *mut c_void) -> f32 {
      let hello: &String = unsafe { transmute(udata) };
      println!("f: {}", hello);
      3.14
    }
    let mut integrand = QuadpackIntegrand{
      f:    integrand_f32,
      data: format!("Hello, world!"),
    };
    let mut scratch = QuadpackScratch::new(4096);
    let res = integrand.integrate_qags(0.0, 1.0, 0.0, epsrel, &mut scratch);
    println!("integral(32): {:?}", res);
    assert!(((res.unwrap().value - 3.14) / 3.14).abs() < 2.0 * epsrel);
  }

  {
    let epsrel = 1.0e-6;
    extern "C" fn integrand_f64(x: *mut f64, udata: *mut c_void) -> f64 {
      /*let hello: &String = unsafe { transmute(udata) };
      println!("f: {}", hello);
      3.14*/
      let x = unsafe { *x };
      let hello: &String = unsafe { transmute(udata) };
      let y = (-x.abs()).exp();
      println!("f: {} x: {:e}, y: {:e}", hello, x, y);
      y
    }
    let mut integrand = QuadpackIntegrand{
      f:    integrand_f64,
      data: format!("Hello, world!"),
    };
    let mut scratch = QuadpackScratch::new(4096);
    let res = integrand.integrate(Bounded(0.0, 1.0), 0.0, epsrel, &mut scratch);
    println!("integral(64): {:?}", res);
    //assert!(((res.unwrap().value - 3.14) / 3.14).abs() < 2.0 * epsrel);
  }

  {
    let epsabs = 0.0;
    let epsrel = 1.0e-6;
    extern "C" fn integrand_f64(x: *mut f64, udata: *mut c_void) -> f64 {
      let x = unsafe { *x };
      let hello: &String = unsafe { transmute(udata) };
      let y = (-x.abs()).exp();
      println!("f: {} x: {:e}, y: {:e}", hello, x, y);
      y
    }
    let mut integrand = QuadpackIntegrand{
      f:    integrand_f64,
      data: format!("Hello, world!"),
    };
    let mut scratch = QuadpackScratch::new(4096);
    let res = integrand.integrate(Infinite, epsabs, epsrel, &mut scratch);
    println!("integral(64): {:?}", res);
    assert!(((res.unwrap().value - 2.0) / 2.0).abs() < 2.0 * epsrel);
  }
}
