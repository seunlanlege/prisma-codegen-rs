/// Prisma Client.
///
/// We re-export them under one crate
pub use prisma_codegen::*;
pub use prisma_derive::*;

pub trait Queryable {
	fn query() -> String;
}

impl<T: Queryable> Queryable for Vec<T> {
	fn query() -> String {
		T::query()
	}
}

impl Queryable for u128 {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for u64 {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for u32 {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for u16 {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for u8 {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for i128 {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for i64 {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for i32 {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for i16 {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for i8 {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for &str {
	fn query() -> String {
		String::new()
	}
}

impl Queryable for String {
	fn query() -> String {
		String::new()
	}
}
