use crate::ui::IsUi;

pub trait IsUiComponent<Ui: IsUi, Settings, ParentActionBuffer> {
    fn render_dyn(
        &mut self,
        settings: &mut Settings,
        hotkey_buffer: &mut Ui::HotkeyBuffer,
        action_buffer: &mut ParentActionBuffer,
        ui: &mut Ui,
    );
}

pub struct UiComponent<Ui: IsUi, Settings, ParentActionBuffer>(
    Box<dyn IsUiComponent<Ui, Settings, ParentActionBuffer>>,
);

impl<Ui: IsUi, Settings, ParentActionBuffer> UiComponent<Ui, Settings, ParentActionBuffer> {
    pub fn ui(
        &mut self,
        settings: &mut Settings,
        hotkey_buffer: &mut Ui::HotkeyBuffer,
        action_buffer: &mut ParentActionBuffer,
        ui: &mut Ui,
    ) {
        self.0
            .render_dyn(settings, hotkey_buffer, action_buffer, ui)
    }
}

impl<Ui: IsUi, UiComponentConfig, ParentActionBuffer>
    UiComponent<Ui, UiComponentConfig, ParentActionBuffer>
{
    pub fn new<UiComponentImpl>(ui_component: UiComponentImpl) -> Self
    where
        UiComponentImpl: IsUiComponent<Ui, UiComponentConfig, ParentActionBuffer> + 'static,
    {
        Self(Box::new(ui_component))
    }
}
