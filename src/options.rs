pub struct FeatureSelections {
    pub selections: Vec<FeatureSelection>,
}

pub struct FeatureSelection {
    pub name: String,
    pub options: Vec<FeatureOption>,
}

pub struct FeatureOption {
    pub name: String,
    pub id: usize,
}

pub trait HasSelection {
    fn feature_id(&self) -> usize;
    fn selections(&self) -> FeatureSelections;
}