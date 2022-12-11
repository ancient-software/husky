use super::*;

impl DeveloperGuiContext {
    pub(super) fn set_presentation(&self, presentation: Presentation) {
        self.presentation_signal.set(presentation);
    }

    pub(super) fn opt_sample_id(&self) -> Option<SampleId> {
        self.presentation_signal.get().opt_sample_id()
    }

    pub(super) fn did_lock_presentation(&mut self, presentation: Presentation) {
        self.presentation_signal.set(presentation);
    }

    pub(super) fn opt_active_trace_id(&self) -> Option<TraceId> {
        self.presentation_signal.get().opt_active_trace_id()
    }

    pub(crate) fn presentation_signal(&self) -> &'static ReadSignal<Presentation> {
        self.presentation_signal
    }
}

fn ask_for_sample_id() -> SampleId {
    let window = web_sys::window().unwrap();
    let mut last_error: Option<String> = None;
    loop {
        let answer = match last_error {
            Some(error) => window.prompt_with_message(&format!("{:?}\ninput id = ", error)),
            None => window.prompt_with_message("input id = "),
        };
        match answer {
            Ok(Some(sample_id_str)) => match sample_id_str.parse::<usize>() {
                Ok(raw) => break SampleId(raw),
                Err(e) => {
                    last_error = Some(format!("expect a valid number, but get {:?} instead", e))
                }
            },
            Ok(None) => last_error = Some(format!("expect a valid number, but get nothing")),
            Err(ref e) => last_error = Some(unsafe { js_sys::JSON::stringify(e) }.unwrap().into()),
        }
    }
}
