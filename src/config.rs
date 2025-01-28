use crate::common::{
    DEFAULT_BREAK_MSG, DEFAULT_COMPLETE_BLOCK, DEFAULT_DELIMITER, DEFAULT_FINISHED_MSG,
    DEFAULT_LEFT_PAD, DEFAULT_NUM_BLOCKS, DEFAULT_PENDING_BLOCK, DEFAULT_RIGHT_PAD,
};

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
            pending_block: DEFAULT_PENDING_BLOCK.into(),
            complete_block: DEFAULT_COMPLETE_BLOCK.into(),
            left_pad: DEFAULT_LEFT_PAD.into(),
            right_pad: DEFAULT_RIGHT_PAD.into(),
            delimiter: DEFAULT_DELIMITER.into(),
            num_blocks: DEFAULT_NUM_BLOCKS,
            finished_msg: DEFAULT_FINISHED_MSG.into(),
            break_msg: DEFAULT_BREAK_MSG.into(),
        }
    }
}
