use displaydoc::Display as DisplayDoc;

#[derive(DisplayDoc, Debug)]
pub enum KernelError {
	/// Kernel shutdown
	Shutdown,

	/// Unknown error
	Unknown,
}
