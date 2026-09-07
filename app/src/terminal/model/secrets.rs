use string_offset::StringRange;
pub use rook_terminal::model::secrets::*;
use rookui::EntityId;

use crate::ai::blocklist::block::TextLocation;
#[derive(Clone, Debug)]
pub struct RichContentSecretTooltipInfo {
    pub secret: String,
    pub secret_range: StringRange,
    pub location: TextLocation,
    pub is_obfuscated: bool,
    pub position_id: String,
    pub view_id: EntityId,
    pub secret_level: SecretLevel,
}
