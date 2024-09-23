pub const DEFAULT_PENDING_BLOCK: &str = "▫";
pub const DEFAULT_COMPLETE_BLOCK: &str = "▪";
pub const DEFAULT_LEFT_PAD: &str = " ";
pub const DEFAULT_RIGHT_PAD: &str = " ";
pub const DEFAULT_DELIMITER: &str = "";
pub const DEFAULT_NUM_BLOCKS: u8 = 10;
pub const DEFAULT_FINISHED_MSG: &str = "done";
pub const DEFAULT_BREAK_MSG: &str = "\\o/";

pub const BREAK_STRING: &str = "break";
pub const STOP_STRING: &str = "stop";

pub struct DisplayConfig {
    pub pending_block: String,
    pub complete_block: String,
    pub left_pad: String,
    pub right_pad: String,
    pub delimiter: String,
    pub num_blocks: u8,
    pub finished_msg: String,
    pub break_msg: String,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            pending_block: DEFAULT_PENDING_BLOCK.to_string(),
            complete_block: DEFAULT_COMPLETE_BLOCK.to_string(),
            left_pad: DEFAULT_LEFT_PAD.to_string(),
            right_pad: DEFAULT_RIGHT_PAD.to_string(),
            delimiter: DEFAULT_DELIMITER.to_string(),
            num_blocks: DEFAULT_NUM_BLOCKS,
            finished_msg: DEFAULT_FINISHED_MSG.to_string(),
            break_msg: DEFAULT_BREAK_MSG.to_string(),
        }
    }
}
