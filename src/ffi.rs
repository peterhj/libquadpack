use libc::*;

#[link(name = "gfortran")]
extern "C" {}

#[no_mangle]
pub extern "C" fn xerror_(msg: *mut c_void, nmsg: *mut c_int, nerr: *mut c_int, level: *mut c_int) {
  panic!("quadpack called xerror");
}

#[link(name = "quadpack_native", kind = "static")]
extern "C" {
  pub fn qagi_(
      f: extern "C" fn (x: *mut f32) -> f32,
      bound: *mut f32,
      inf: *mut c_int,
      epsabs: *mut f32,
      epsrel: *mut f32,
      result: *mut f32,
      neval: *mut c_int,
      ier: *mut c_int,
      limit: *mut c_int,
      lenw: *mut c_int,
      last: *mut c_int,
      iwork: *mut c_int,
      work: *mut f32,
  );
  pub fn dqagi_(
      f: extern "C" fn (x: *mut f64) -> f64,
      bound: *mut f64,
      inf: *mut c_int,
      epsabs: *mut f64,
      epsrel: *mut f64,
      result: *mut f64,
      neval: *mut c_int,
      ier: *mut c_int,
      limit: *mut c_int,
      lenw: *mut c_int,
      last: *mut c_int,
      iwork: *mut c_int,
      work: *mut f64,
  );
  pub fn qags_(
      f: extern "C" fn (x: *mut f32) -> f32,
      a: *mut f32,
      b: *mut f32,
      epsabs: *mut f32,
      epsrel: *mut f32,
      result: *mut f32,
      abserr: *mut f32,
      neval: *mut c_int,
      ier: *mut c_int,
      limit: *mut c_int,
      lenw: *mut c_int,
      last: *mut c_int,
      iwork: *mut c_int,
      work: *mut f32,
  );
  pub fn dqags_(
      f: extern "C" fn (x: *mut f64) -> f64,
      a: *mut f64,
      b: *mut f64,
      epsabs: *mut f64,
      epsrel: *mut f64,
      result: *mut f64,
      abserr: *mut f64,
      neval: *mut c_int,
      ier: *mut c_int,
      limit: *mut c_int,
      lenw: *mut c_int,
      last: *mut c_int,
      iwork: *mut c_int,
      work: *mut f64,
  );
  /*pub fn dqags_data_(
      f: extern "C" fn (x: *mut f64, data: *mut c_void),
      data: *mut c_void,
      a: *mut f64,
      b: *mut f64,
      epsabs: *mut f64,
      epsrel: *mut f64,
      result: *mut f64,
      neval: *mut c_int,
      ier: *mut c_int,
      limit: *mut c_int,
      lenw: *mut c_int,
      last: *mut c_int,
      iwork: *mut c_int,
      work: *mut f64,
  );*/
}
