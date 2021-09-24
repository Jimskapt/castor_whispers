#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Settings {
	pub replacements: SettingsReplacements,
	pub ignore_rows: Option<SettingsIgnoreRows>,
}
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct SettingsReplacements {
	pub comment: Option<Vec<(String, String)>>,
	pub only_commented: Option<Vec<(String, String)>>,
}
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct SettingsIgnoreRows {
	pub only_commented: Option<Vec<String>>,
}
