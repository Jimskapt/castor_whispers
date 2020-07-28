#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Settings {
	pub replacements: SettingsReplacements,
}
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct SettingsReplacements {
	pub comment: Option<Vec<(String, String)>>,
	pub only_commented: Option<Vec<(String, String)>>,
}
