use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct Row(pub u32); // raw is 0 based

impl HuskyDisplay for Row {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}row {: <4}{}",
                husky_print_utils::YELLOW,
                self.0 + 1,
                husky_print_utils::RESET
            )
            .unwrap();
        } else {
            write!(result, "row {: <4}", self.0 + 1,).unwrap();
        }
    }
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0 + 1).fmt(f)
    }
}

impl From<u32> for Row {
    fn from(base0: u32) -> Self {
        Row(base0)
    }
}

impl From<usize> for Row {
    fn from(base0: usize) -> Self {
        Row(<usize as TryInto<u32>>::try_into(base0).expect("success"))
    }
}

impl From<i32> for Row {
    fn from(base0: i32) -> Self {
        assert!(base0 >= 0);
        Row(base0 as u32)
    }
}
