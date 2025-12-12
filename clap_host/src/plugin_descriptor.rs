use clack_host::factory;
use std::{
	ffi::CStr,
	fmt::{Display, Formatter},
	str,
	sync::Arc,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PluginDescriptor {
	pub name: Arc<str>,
	pub id: Arc<CStr>,
}

impl Display for PluginDescriptor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.name.fmt(f)
	}
}

impl TryFrom<factory::PluginDescriptor<'_>> for PluginDescriptor {
	type Error = Option<str::Utf8Error>;

	fn try_from(value: factory::PluginDescriptor<'_>) -> Result<Self, Self::Error> {
		Ok(Self {
			name: value.name().ok_or(None)?.to_str()?.into(),
			id: value.id().ok_or(None)?.into(),
		})
	}
}
