use crate::config::colors::{deserialize_to_arr, deserialize_to_arr_opt, ColorArray};
use crate::config::default_bool_true;
use serde::{Deserialize, Serialize};

#[inline]
pub fn default_unfocused_split_opacity() -> f32 {
    0.7
}

/// Clamp `unfocused_split_opacity` to `[0.15, 1.0]`.
///
/// A value of `0.0` makes the inactive pane invisible, which is never what
/// the user wants; the lower bound keeps the pane legible at the darkest
/// setting.
#[inline]
pub fn clamp_unfocused_split_opacity(v: f32) -> f32 {
    v.clamp(0.15, 1.0)
}

#[inline]
pub fn default_floating_sidebar_opacity() -> f32 {
    0.92
}

#[inline]
pub fn clamp_floating_sidebar_opacity(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum NavigationMode {
    #[serde(alias = "plain")]
    Plain,
    #[serde(alias = "tab")]
    Tab,
    #[serde(alias = "floating-sidebar", alias = "floatingsidebar")]
    FloatingSidebar,
    #[cfg(target_os = "macos")]
    #[serde(alias = "nativetab")]
    NativeTab,
}

#[allow(clippy::derivable_impls)]
impl Default for NavigationMode {
    fn default() -> NavigationMode {
        #[cfg(target_os = "macos")]
        {
            // Use Tab for full GPU rendering
            NavigationMode::Tab
        }

        #[cfg(not(target_os = "macos"))]
        NavigationMode::Tab
    }
}

impl NavigationMode {
    const PLAIN_STR: &'static str = "Plain";
    const TAB_STR: &'static str = "Tab";
    const FLOATING_SIDEBAR_STR: &'static str = "FloatingSidebar";
    #[cfg(target_os = "macos")]
    const NATIVE_TAB_STR: &'static str = "NativeTab";

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Plain => Self::PLAIN_STR,
            Self::Tab => Self::TAB_STR,
            Self::FloatingSidebar => Self::FLOATING_SIDEBAR_STR,
            #[cfg(target_os = "macos")]
            Self::NativeTab => Self::NATIVE_TAB_STR,
        }
    }
}

#[inline]
pub fn modes_as_vec_string() -> Vec<String> {
    [
        NavigationMode::Plain,
        NavigationMode::Tab,
        NavigationMode::FloatingSidebar,
        #[cfg(target_os = "macos")]
        NavigationMode::NativeTab,
    ]
    .iter()
    .map(|navigation_mode| navigation_mode.to_string())
    .collect()
}

impl std::fmt::Display for NavigationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseNavigationModeError;

impl std::str::FromStr for NavigationMode {
    type Err = ParseNavigationModeError;

    fn from_str(s: &str) -> Result<NavigationMode, ParseNavigationModeError> {
        match s {
            Self::PLAIN_STR => Ok(NavigationMode::Plain),
            Self::TAB_STR => Ok(NavigationMode::Tab),
            Self::FLOATING_SIDEBAR_STR => Ok(NavigationMode::FloatingSidebar),
            #[cfg(target_os = "macos")]
            Self::NATIVE_TAB_STR => Ok(NavigationMode::NativeTab),
            _ => Ok(NavigationMode::default()),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ColorAutomation {
    #[serde(default = "String::new")]
    pub program: String,
    #[serde(default = "String::new")]
    pub path: String,
    #[serde(
        deserialize_with = "deserialize_to_arr",
        default = "crate::config::colors::defaults::tabs"
    )]
    pub color: ColorArray,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Navigation {
    #[serde(default = "NavigationMode::default")]
    pub mode: NavigationMode,
    #[serde(
        default = "Vec::default",
        rename = "color-automation",
        skip_serializing
    )]
    pub color_automation: Vec<ColorAutomation>,
    #[serde(default = "bool::default", skip_serializing)]
    pub clickable: bool,
    #[serde(
        default = "default_bool_true",
        rename = "current-working-directory",
        alias = "cwd"
    )]
    pub current_working_directory: bool,
    #[serde(default = "bool::default", rename = "use-terminal-title")]
    pub use_terminal_title: bool,
    #[serde(default = "default_bool_true", rename = "hide-if-single")]
    pub hide_if_single: bool,
    #[serde(default = "default_bool_true", rename = "use-split")]
    pub use_split: bool,
    #[serde(default = "default_bool_true", rename = "open-config-with-split")]
    pub open_config_with_split: bool,
    /// The opacity level of an unfocused split. A value of `1.0` disables the
    /// dim; lower values fade the pane out. Clamped to `[0.15, 1.0]` at load
    /// time — a value of `0` makes the pane invisible, which is never useful.
    #[serde(
        default = "default_unfocused_split_opacity",
        rename = "unfocused-split-opacity"
    )]
    pub unfocused_split_opacity: f32,
    /// The color used to dim an unfocused split. The overlay's alpha is
    /// derived from `unfocused_split_opacity` — this field is an RGB tint
    /// only. When unset, the terminal's background color is used.
    #[serde(
        default = "Option::default",
        deserialize_with = "deserialize_to_arr_opt",
        rename = "unfocused-split-fill"
    )]
    pub unfocused_split_fill: Option<ColorArray>,
    #[serde(
        default = "default_floating_sidebar_opacity",
        rename = "floating-sidebar-opacity"
    )]
    pub floating_sidebar_opacity: f32,
}

impl Default for Navigation {
    fn default() -> Navigation {
        Navigation {
            mode: NavigationMode::default(),
            color_automation: Vec::default(),
            clickable: false,
            current_working_directory: true,
            use_terminal_title: false,
            hide_if_single: true,
            use_split: true,
            unfocused_split_opacity: default_unfocused_split_opacity(),
            unfocused_split_fill: None,
            floating_sidebar_opacity: default_floating_sidebar_opacity(),
            open_config_with_split: true,
        }
    }
}

impl Navigation {
    #[inline]
    pub fn is_native(&self) -> bool {
        #[cfg(target_os = "macos")]
        {
            self.mode == NavigationMode::NativeTab
        }

        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    }

    #[inline]
    pub fn has_navigation_key_bindings(&self) -> bool {
        self.mode != NavigationMode::Plain
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.mode == NavigationMode::Tab
    }

    #[inline]
    pub fn is_floating_sidebar(&self) -> bool {
        self.mode == NavigationMode::FloatingSidebar
    }

    #[inline]
    pub fn has_rio_rendered_tabs(&self) -> bool {
        matches!(
            self.mode,
            NavigationMode::Tab | NavigationMode::FloatingSidebar
        )
    }

    /// Whether the rio-rendered tab strip ("island") is actually painted
    /// this frame. Mirrors the gate at `island.rs:358` — input layers
    /// (click routing, cursor override) must agree with the renderer so
    /// the empty band over a hidden island doesn't intercept events.
    #[inline]
    pub fn island_visible(&self, num_tabs: usize) -> bool {
        self.is_enabled() && !(self.hide_if_single && num_tabs == 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::colors::hex_to_color_arr;
    use crate::config::navigation::{Navigation, NavigationMode};
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize, PartialEq)]
    struct Root {
        #[serde(default = "Navigation::default")]
        navigation: Navigation,
    }

    #[test]
    fn test_plain() {
        let content = r#"
            [navigation]
            mode = 'Plain'
        "#;

        let decoded = toml::from_str::<Root>(content).unwrap();
        assert_eq!(decoded.navigation.mode, NavigationMode::Plain);
        assert!(!decoded.navigation.clickable);
        assert!(decoded.navigation.color_automation.is_empty());
    }

    #[test]
    fn test_tab() {
        let content = r#"
            [navigation]
            mode = 'Tab'
        "#;

        let decoded = toml::from_str::<Root>(content).unwrap();
        assert_eq!(decoded.navigation.mode, NavigationMode::Tab);
        assert!(!decoded.navigation.clickable);
        assert!(decoded.navigation.color_automation.is_empty());
    }

    #[test]
    fn test_floating_sidebar() {
        let content = r#"
            [navigation]
            mode = 'FloatingSidebar'
            floating-sidebar-opacity = 0.72
        "#;

        let decoded = toml::from_str::<Root>(content).unwrap();
        assert_eq!(decoded.navigation.mode, NavigationMode::FloatingSidebar);
        assert_eq!(decoded.navigation.floating_sidebar_opacity, 0.72);
        assert!(decoded.navigation.has_navigation_key_bindings());
        assert!(decoded.navigation.has_rio_rendered_tabs());
        assert!(decoded.navigation.is_floating_sidebar());
        assert!(!decoded.navigation.is_enabled());
    }

    #[test]
    fn test_color_automation() {
        let content = r#"
            [navigation]
            mode = 'Tab'
            color-automation = [
                { program = 'vim', color = '#333333' }
            ]
        "#;

        let decoded = toml::from_str::<Root>(content).unwrap();
        assert_eq!(decoded.navigation.mode, NavigationMode::Tab);
        assert!(!decoded.navigation.clickable);
        assert!(!decoded.navigation.color_automation.is_empty());
        assert_eq!(
            decoded.navigation.color_automation[0].program,
            "vim".to_string()
        );
        assert_eq!(decoded.navigation.color_automation[0].path, String::new());
        assert_eq!(
            decoded.navigation.color_automation[0].color,
            hex_to_color_arr("#333333")
        );
    }

    #[test]
    fn test_color_automation_arr() {
        let content = r#"
            [navigation]
            mode = 'Tab'
            color-automation = [
                { program = 'ssh', color = '#F1F1F1' },
                { program = 'tmux', color = '#333333' },
                { path = '/home', color = '#ffffff' },
                { program = 'nvim', path = '/usr', color = '#00b952' },
            ]
        "#;

        let decoded = toml::from_str::<Root>(content).unwrap();
        assert_eq!(decoded.navigation.mode, NavigationMode::Tab);
        assert!(!decoded.navigation.clickable);
        assert!(!decoded.navigation.color_automation.is_empty());

        assert_eq!(
            decoded.navigation.color_automation[0].program,
            "ssh".to_string()
        );
        assert_eq!(decoded.navigation.color_automation[0].path, String::new());
        assert_eq!(
            decoded.navigation.color_automation[0].color,
            hex_to_color_arr("#F1F1F1")
        );

        assert_eq!(
            decoded.navigation.color_automation[1].program,
            "tmux".to_string()
        );
        assert_eq!(decoded.navigation.color_automation[1].path, String::new());
        assert_eq!(
            decoded.navigation.color_automation[1].color,
            hex_to_color_arr("#333333")
        );

        assert_eq!(
            decoded.navigation.color_automation[2].program,
            String::new()
        );
        assert_eq!(
            decoded.navigation.color_automation[2].path,
            "/home".to_string()
        );
        assert_eq!(
            decoded.navigation.color_automation[2].color,
            hex_to_color_arr("#ffffff")
        );

        assert_eq!(
            decoded.navigation.color_automation[3].program,
            "nvim".to_string()
        );
        assert_eq!(
            decoded.navigation.color_automation[3].path,
            "/usr".to_string()
        );
        assert_eq!(
            decoded.navigation.color_automation[3].color,
            hex_to_color_arr("#00b952")
        );
    }
}
