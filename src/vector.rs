extern crate num;
extern crate simd;

use self::num::traits::Float;
use std::ops::{Index, Add};
use self::simd::f32x4;

pub struct Vector<T: Float> {
  data: Vec<T>
}

impl<T: Float> Vector<T> {
  pub fn new(elems: &[T]) -> Vector<T> {
    let mut v = Vector::<T> { data: Vec::new() };
    v.data.extend(elems);
    v
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.data.len()
  }
}

impl<T: Float> Index<usize> for Vector<T> {
  type Output = T;

  #[inline]
  fn index<'a>(&'a self, index: usize) -> &'a T {
    &self.data[index]
  }
}

impl<T: Float> Clone for Vector<T> {
  fn clone(&self) -> Vector<T> {
    Vector::<T> {
      data: self.data.clone()
    }
  }

  fn clone_from(&mut self, source: &Vector<T>) {
    self.data = source.data.clone();
  }
}

impl Add<Vector<f32>> for Vector<f32> {
  type Output = Result<Vector<f32>, String>;

  fn add(self, rhs: Vector<f32>) -> Result<Vector<f32>, String> {
    if self.data.len() == rhs.data.len() {
      let mut new_vec = Vec::new();
      let lhs_data = self.data.as_slice();
      let rhs_data = rhs.data.as_slice();
      for i in (0..self.data.len()).step_by(4) {
        let mut reg_len = 4;
        let mut reg1: f32x4;
        let mut reg2: f32x4;
        if self.data.len() - i < 4 {
          let (mut x1, mut x2, mut x3) = (0.0_f32, 0.0_f32, 0.0_f32);
          let (mut y1, mut y2, mut y3) = (0.0_f32, 0.0_f32, 0.0_f32);
          reg_len = self.data.len() - i;
          for j in i..self.data.len() {
            let diff = self.data.len() - j;
            match diff {
              1 => { x1 = lhs_data[j]; y1 = rhs_data[j] },
              2 => { x2 = lhs_data[j]; y2 = rhs_data[j] },
              3 => { x3 = lhs_data[j]; y3 = rhs_data[j] },
              _ => { unreachable!() }
            }
          }
          reg1 = f32x4::new(x1, x2, x3, 0.0_f32);
          reg2 = f32x4::new(y1, y2, y3, 0.0_f32);
        } else {
          reg1 = f32x4::load(lhs_data, i);
          reg2 = f32x4::load(rhs_data, i);
        }
        let res = reg1 + reg2;
        for j in 0..reg_len {
          new_vec.push(res.extract(j as u32));
        }
      }
      Ok(Vector::<f32> { data: new_vec })
    } else {
      Err("Vectors are not conformable for addition.".to_string())
    }
  }
}
