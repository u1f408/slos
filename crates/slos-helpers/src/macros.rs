/// Get the name of the current function as a `&'static str`.
///
/// Gently lifted from https://stackoverflow.com/a/63904992
#[macro_export]
macro_rules! function {
	() => {{
		fn type_name_of<T>(_: T) -> &'static str {
			core::any::type_name::<T>()
		}

		fn f() {}
		let name = type_name_of(f);

		// Find and cut the rest of the path
		match &name[..name.len() - 3].rfind(':') {
			Some(pos) => &name[pos + 1..name.len() - 3],
			None => &name[..name.len() - 3],
		}
	}};
}
