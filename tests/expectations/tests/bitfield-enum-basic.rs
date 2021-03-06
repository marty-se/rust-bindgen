/* automatically generated by rust-bindgen */

#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]

pub const Foo_Bar: Foo = Foo(2);
pub const Foo_Baz: Foo = Foo(4);
pub const Foo_Duplicated: Foo = Foo(4);
pub const Foo_Negative: Foo = Foo(-3);
impl ::std::ops::BitOr<Foo> for Foo {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Foo(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for Foo {
    #[inline]
    fn bitor_assign(&mut self, rhs: Foo) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<Foo> for Foo {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Foo(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for Foo {
    #[inline]
    fn bitand_assign(&mut self, rhs: Foo) {
        self.0 &= rhs.0;
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Foo(pub i32);
pub const Buz_Bar: Buz = Buz(2);
pub const Buz_Baz: Buz = Buz(4);
pub const Buz_Duplicated: Buz = Buz(4);
pub const Buz_Negative: Buz = Buz(-3);
impl ::std::ops::BitOr<Buz> for Buz {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Buz(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for Buz {
    #[inline]
    fn bitor_assign(&mut self, rhs: Buz) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<Buz> for Buz {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Buz(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for Buz {
    #[inline]
    fn bitand_assign(&mut self, rhs: Buz) {
        self.0 &= rhs.0;
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Buz(pub i8);
pub const NS_FOO: _bindgen_ty_1 = _bindgen_ty_1(1);
pub const NS_BAR: _bindgen_ty_1 = _bindgen_ty_1(2);
impl ::std::ops::BitOr<_bindgen_ty_1> for _bindgen_ty_1 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        _bindgen_ty_1(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for _bindgen_ty_1 {
    #[inline]
    fn bitor_assign(&mut self, rhs: _bindgen_ty_1) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<_bindgen_ty_1> for _bindgen_ty_1 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        _bindgen_ty_1(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for _bindgen_ty_1 {
    #[inline]
    fn bitand_assign(&mut self, rhs: _bindgen_ty_1) {
        self.0 &= rhs.0;
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct _bindgen_ty_1(pub u32);
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Dummy {
    pub _address: u8,
}
pub const Dummy_DUMMY_FOO: Dummy__bindgen_ty_1 = Dummy__bindgen_ty_1(1);
pub const Dummy_DUMMY_BAR: Dummy__bindgen_ty_1 = Dummy__bindgen_ty_1(2);
impl ::std::ops::BitOr<Dummy__bindgen_ty_1> for Dummy__bindgen_ty_1 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Dummy__bindgen_ty_1(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for Dummy__bindgen_ty_1 {
    #[inline]
    fn bitor_assign(&mut self, rhs: Dummy__bindgen_ty_1) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<Dummy__bindgen_ty_1> for Dummy__bindgen_ty_1 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Dummy__bindgen_ty_1(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for Dummy__bindgen_ty_1 {
    #[inline]
    fn bitand_assign(&mut self, rhs: Dummy__bindgen_ty_1) {
        self.0 &= rhs.0;
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Dummy__bindgen_ty_1(pub u32);
#[test]
fn bindgen_test_layout_Dummy() {
    assert_eq!(
        ::std::mem::size_of::<Dummy>(),
        1usize,
        concat!("Size of: ", stringify!(Dummy))
    );
    assert_eq!(
        ::std::mem::align_of::<Dummy>(),
        1usize,
        concat!("Alignment of ", stringify!(Dummy))
    );
}
