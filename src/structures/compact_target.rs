use crate::utils::push_u32;
use bitcoin::CompactTarget;
use bitvm::treepp::*;

pub struct CompactTargetGadget;

impl CompactTargetGadget {
    pub fn from_constant(compact_target: &CompactTarget) -> Script {
        let v = compact_target.to_consensus();
        push_u32(v)
    }
}
