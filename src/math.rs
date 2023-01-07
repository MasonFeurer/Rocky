use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

pub trait VecType: Clone + Copy {}
impl<T: Clone + Copy> VecType for T {}

macro_rules! impl_op {
	($name:ident{$($field:ident),*},$op_trait:ident,$op_fn:ident,$op_tt:tt) => {
		impl<T: VecType + $op_trait<Output = T>> $op_trait<Self> for $name<T> {
			type Output = Self;
			fn $op_fn(self, rhs: Self) -> Self {
				Self {
					$($field: self.$field $op_tt rhs.$field),*
				}
			}
		}
		impl<T: VecType + $op_trait<Output = T>> $op_trait<T> for $name<T> {
			type Output = Self;
			fn $op_fn(self, rhs: T) -> Self {
				Self {
					$($field: self.$field $op_tt rhs),*
				}
			}
		}
	}
}

macro_rules! define_vec {
	($name:ident{$($field:ident),*};$size:literal) => {
		#[derive(Clone, Copy, Debug, PartialEq, Eq)]
		#[repr(C)]
		pub struct $name<T: VecType> {
			$(pub $field: T),*
		}
		impl<T: VecType> $name<T> {
			#[inline(always)]
			pub const fn new($($field: T),*) -> Self {
				Self { $($field),* }
			}

			#[inline(always)]
			pub const fn all(v: T) -> Self {
				Self { $($field: v),* }
			}

			#[inline(always)]
			pub const fn pod(self) -> [T; $size] {
				[$(self.$field),*]
			}

			#[inline(always)]
			pub fn map<E: VecType>(&self, f: impl Fn(T) -> E) -> $name<E> {
				$name { $($field: f(self.$field)),* }
			}
		}

		impl_op!($name{$($field),*},Add,add,+);
		impl_op!($name{$($field),*},Sub,sub,-);
		impl_op!($name{$($field),*},Mul,mul,*);
		impl_op!($name{$($field),*},Div,div,/);
	}
}

define_vec!(Vec2 { x, y };2);
define_vec!(Vec3 { x, y, z };3);
define_vec!(Vec4 { x, y, z, w };4);

impl Vec2<f32> {
    #[inline(always)]
    pub fn len_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    #[inline(always)]
    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    #[inline(always)]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline(always)]
    pub fn cross(self, rhs: Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    #[inline(always)]
    pub fn norm(self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }
}
impl Vec3<f32> {
    #[inline(always)]
    pub fn len_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    #[inline(always)]
    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    #[inline(always)]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline(always)]
    pub fn norm(self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}
impl Vec4<f32> {
    #[inline(always)]
    pub fn len_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
    #[inline(always)]
    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    #[inline(always)]
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    #[inline(always)]
    pub fn norm(self) -> Self {
        let len = self.len();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len,
        }
    }
}

// ---- FROM TUPLES & ARRAYS
impl<T: Copy> From<(T, T)> for Vec2<T> {
    fn from(v: (T, T)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}
impl<T: Copy> From<[T; 2]> for Vec2<T> {
    fn from(v: [T; 2]) -> Self {
        Self { x: v[0], y: v[1] }
    }
}

impl<T: Copy> From<(T, T, T)> for Vec3<T> {
    fn from(v: (T, T, T)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}
impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(v: [T; 3]) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}
impl<T: Copy> From<Vec3<T>> for [T; 3] {
    fn from(v: Vec3<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

impl<T: Copy> From<(T, T, T, T)> for Vec4<T> {
    fn from(v: (T, T, T, T)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
            w: v.3,
        }
    }
}
impl<T: Copy> From<[T; 4]> for Vec4<T> {
    fn from(v: [T; 4]) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
            w: v[3],
        }
    }
}

// ---- MATRIX 4x4 ----
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat4(pub [[f32; 4]; 4]);
impl Mat4 {
    #[inline(always)]
    pub fn pod(&self) -> [[f32; 4]; 4] {
        self.0.clone()
    }

    #[inline(always)]
    pub const fn empty() -> Self {
        Self([[0.0; 4]; 4])
    }
    #[inline(always)]
    pub const fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[inline(always)] // column-major
    pub const fn get(&self, col: usize, row: usize) -> f32 {
        self.0[row][col]
    }
    #[inline(always)] // column-major
    pub fn set(&mut self, col: usize, row: usize, value: f32) {
        self.0[row][col] = value;
    }

    #[inline(always)]
    pub fn get_row(&self, row: usize) -> Vec4<f32> {
        Vec4 {
            x: self.get(0, row),
            y: self.get(1, row),
            z: self.get(2, row),
            w: self.get(3, row),
        }
    }
    #[inline(always)]
    pub fn get_col(&self, col: usize) -> Vec4<f32> {
        Vec4 {
            x: self.get(col, 0),
            y: self.get(col, 1),
            z: self.get(col, 2),
            w: self.get(col, 3),
        }
    }

    /// Creates a 4x4 matrix that rotates a `Vec3` about the X axis.
    /// Expects angle `a` to be in radians.
    pub fn x_rotation(a: f32) -> Self {
        let (s, c) = (a.sin(), a.cos());
        let mut out = Self::identity();
        out.set(1, 1, c);
        out.set(2, 2, c);
        out.set(2, 1, s);
        out.set(1, 2, -s);
        out
    }
    /// Creates a 4x4 matrix that rotates a `Vec3` about the Y axis.
    /// Expects angle `a` to be in radians.
    pub fn y_rotation(a: f32) -> Self {
        let (s, c) = (a.sin(), a.cos());
        let mut out = Self::identity();
        out.set(0, 0, c);
        out.set(2, 0, s);
        out.set(0, 2, -s);
        out.set(2, 2, c);
        out
    }
    /// Creates a 4x4 matrix that rotates a `Vec3` about the Z axis.
    /// Expects angle `a` to be in radians.
    pub fn z_rotation(a: f32) -> Self {
        let (s, c) = (a.sin(), a.cos());
        let mut out = Self::identity();
        out.set(0, 0, c);
        out.set(1, 0, s);
        out.set(0, 1, s);
        out.set(1, 1, c);
        out
    }

    /// Creates a 4x4 matrix that rotates a `Vec3` by `rot`, translates by `trans`, and scales by `scale`.
    /// Expects `fov` elements to be in radians.
    pub fn transformation(trans: Vec3<f32>, rot: Vec3<f32>, scale: Vec3<f32>) -> Self {
        Self::translation(trans)
            * Self::x_rotation(rot.x)
            * Self::y_rotation(rot.y)
            * Self::z_rotation(rot.z)
            * Self::scaling(scale)
    }

    /// Creates a 4x4 matrix that projects a `Vec3` in world space, to screen space.
    /// expects `fov` to be in radians.
    pub fn projection(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let x_scale = 1.0 / (fov / 2.0).tan();
        let y_scale = x_scale * aspect;
        let range = far - near;

        let mut out = Self::empty();
        out.set(0, 0, x_scale);
        out.set(1, 1, y_scale);
        out.set(2, 2, -((far + near) / range));
        out.set(3, 2, -1.0);
        out.set(2, 3, -((2.0 * near * far) / range));
        out
    }

    /// Creates a 4x4 matrix that rotates a `Vec3` by `rot`, and translates it by the negative of `pos`.
    /// Expects `rot` elements to be in radians.
    pub fn view(pos: Vec3<f32>, rot: Vec3<f32>) -> Self {
        Self::x_rotation(rot.x)
            * Self::y_rotation(rot.y)
            * Self::z_rotation(rot.z)
            * Self::translation(pos * -1.0)
    }

    /// Creates a 4x4 matrix that translates a `Vec3` by `t`.
    pub fn translation(t: Vec3<f32>) -> Self {
        let mut out = Self::identity();
        out.set(0, 3, t.x);
        out.set(1, 3, t.y);
        out.set(2, 3, t.z);
        out
    }

    /// Creates a 4x4 matrix that scales a `Vec3` by `scale`
    pub fn scaling(scale: Vec3<f32>) -> Self {
        let mut out = Self::identity();
        out.set(0, 0, scale.x);
        out.set(1, 1, scale.y);
        out.set(2, 2, scale.z);
        out
    }
}
impl std::ops::Mul for Mat4 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let mut out = Self::empty();
        out.set(0, 0, self.get_col(0).dot(other.get_row(0)) as f32);
        out.set(1, 0, self.get_col(1).dot(other.get_row(0)) as f32);
        out.set(2, 0, self.get_col(2).dot(other.get_row(0)) as f32);
        out.set(3, 0, self.get_col(3).dot(other.get_row(0)) as f32);

        out.set(0, 1, self.get_col(0).dot(other.get_row(1)) as f32);
        out.set(1, 1, self.get_col(1).dot(other.get_row(1)) as f32);
        out.set(2, 1, self.get_col(2).dot(other.get_row(1)) as f32);
        out.set(3, 1, self.get_col(3).dot(other.get_row(1)) as f32);

        out.set(0, 2, self.get_col(0).dot(other.get_row(2)) as f32);
        out.set(1, 2, self.get_col(1).dot(other.get_row(2)) as f32);
        out.set(2, 2, self.get_col(2).dot(other.get_row(2)) as f32);
        out.set(3, 2, self.get_col(3).dot(other.get_row(2)) as f32);

        out.set(0, 3, self.get_col(0).dot(other.get_row(3)) as f32);
        out.set(1, 3, self.get_col(1).dot(other.get_row(3)) as f32);
        out.set(2, 3, self.get_col(2).dot(other.get_row(3)) as f32);
        out.set(3, 3, self.get_col(3).dot(other.get_row(3)) as f32);
        out
    }
}
