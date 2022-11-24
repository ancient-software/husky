use super::*;
use husky_data_viewer::HuskyDataViewer;

impl<'temp, 'eval: 'temp> FeatureEvaluator<'temp, 'eval> {
    pub(super) fn serialize(&self, value: &__Register<'eval>, ty: Term) -> serde_json::Value {
        let ty_data_viewer: Arc<HuskyDataViewer> = self.db.ty_data_viewer(ty);
        ty_data_viewer.serialize(self.db.upcast(), value)
    }
}
