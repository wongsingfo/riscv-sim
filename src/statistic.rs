
#[derive(Default)]
pub struct Statistic {
    pub cycle: u64,
    pub num_inst: u64,
    pub num_branch: u64,
    pub num_mis_pred: u64,
    pub num_data_hazard: u64,
}

impl Statistic {
    pub fn get_cpi(&self) -> f32 {
        (self.cycle as f32) / (self.num_inst as f32)
    }

    pub fn get_pred_accuracy(&self) -> f32 {
        (self.num_mis_pred as f32) / (self.num_branch as f32)
    }
}
