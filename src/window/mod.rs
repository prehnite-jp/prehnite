pub mod license_info;
pub mod main_window;
pub mod settings;
pub mod version_info;

/// Contextにこの構造体が登録されている場合、それはメインウインドウとみなされます。
pub struct MainWindowMarker;
