use vengine_rs::compute::compute_stage::VEComputeStage;
use vengine_rs::graphics::render_stage::VERenderStage;

pub struct CloudGenerator {
    pub render_stage_low_freq: VERenderStage,
    pub compute_stage_hi_freq: VEComputeStage,
}
