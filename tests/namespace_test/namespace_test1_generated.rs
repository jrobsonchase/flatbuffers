// automatically generated by the FlatBuffers compiler, do not modify


#![allow(unused_imports, dead_code)]


extern crate core;

use self::core::mem;
use self::core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod namespace_a {


  extern crate core;

  use self::core::mem;
  use self::core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;
#[allow(unused_imports, dead_code)]
pub mod namespace_b {


  extern crate core;

  use self::core::mem;
  use self::core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[deprecated(since = "1.13", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_ENUM_IN_NESTED_NS: i8 = 0;
#[deprecated(since = "1.13", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_ENUM_IN_NESTED_NS: i8 = 2;
#[deprecated(since = "1.13", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_ENUM_IN_NESTED_NS: [EnumInNestedNS; 3] = [
  EnumInNestedNS::A,
  EnumInNestedNS::B,
  EnumInNestedNS::C,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EnumInNestedNS(pub i8);
#[allow(non_upper_case_globals)]
impl EnumInNestedNS {
  pub const A: Self = Self(0);
  pub const B: Self = Self(1);
  pub const C: Self = Self(2);

  pub const ENUM_MIN: i8 = 0;
  pub const ENUM_MAX: i8 = 2;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::A,
    Self::B,
    Self::C,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::A => Some("A"),
      Self::B => Some("B"),
      Self::C => Some("C"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for EnumInNestedNS {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for EnumInNestedNS {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self(flatbuffers::read_scalar_at::<i8>(buf, loc))
  }
}

impl flatbuffers::Push for EnumInNestedNS {
    type Output = EnumInNestedNS;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<i8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for EnumInNestedNS {
  #[inline]
  fn to_little_endian(self) -> Self {
    Self(i8::to_le(self.0))
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    Self(i8::from_le(self.0))
  }
}

// struct StructInNestedNS, aligned to 4
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq)]
pub struct StructInNestedNS {
  a_: i32,
  b_: i32,
} // pub struct StructInNestedNS
impl std::fmt::Debug for StructInNestedNS {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("StructInNestedNS")
      .field("a", &self.a())
      .field("b", &self.b())
      .finish()
  }
}

impl flatbuffers::SafeSliceAccess for StructInNestedNS {}
impl<'a> flatbuffers::Follow<'a> for StructInNestedNS {
  type Inner = &'a StructInNestedNS;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a StructInNestedNS>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a StructInNestedNS {
  type Inner = &'a StructInNestedNS;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<StructInNestedNS>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for StructInNestedNS {
    type Output = StructInNestedNS;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            self::core::slice::from_raw_parts(self as *const StructInNestedNS as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b StructInNestedNS {
    type Output = StructInNestedNS;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            self::core::slice::from_raw_parts(*self as *const StructInNestedNS as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl StructInNestedNS {
  pub fn new(_a: i32, _b: i32) -> Self {
    StructInNestedNS {
      a_: _a.to_little_endian(),
      b_: _b.to_little_endian(),

    }
  }
    pub const fn get_fully_qualified_name() -> &'static str {
        "NamespaceA.NamespaceB.StructInNestedNS"
    }

  pub fn a(&self) -> i32 {
    self.a_.from_little_endian()
  }
  pub fn b(&self) -> i32 {
    self.b_.from_little_endian()
  }
}

pub enum TableInNestedNSOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TableInNestedNS<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TableInNestedNS<'a> {
    type Inner = TableInNestedNS<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> TableInNestedNS<'a> {
    pub const fn get_fully_qualified_name() -> &'static str {
        "NamespaceA.NamespaceB.TableInNestedNS"
    }

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TableInNestedNS {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TableInNestedNSArgs) -> flatbuffers::WIPOffset<TableInNestedNS<'bldr>> {
      let mut builder = TableInNestedNSBuilder::new(_fbb);
      builder.add_foo(args.foo);
      builder.finish()
    }

    pub const VT_FOO: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn foo(&self) -> i32 {
    self._tab.get::<i32>(TableInNestedNS::VT_FOO, Some(0)).unwrap()
  }
}

pub struct TableInNestedNSArgs {
    pub foo: i32,
}
impl<'a> Default for TableInNestedNSArgs {
    #[inline]
    fn default() -> Self {
        TableInNestedNSArgs {
            foo: 0,
        }
    }
}
pub struct TableInNestedNSBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TableInNestedNSBuilder<'a, 'b> {
  #[inline]
  pub fn add_foo(&mut self, foo: i32) {
    self.fbb_.push_slot::<i32>(TableInNestedNS::VT_FOO, foo, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TableInNestedNSBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TableInNestedNSBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TableInNestedNS<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for TableInNestedNS<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("TableInNestedNS");
      ds.field("foo", &self.foo());
      ds.finish()
  }
}
}  // pub mod NamespaceB
}  // pub mod NamespaceA

