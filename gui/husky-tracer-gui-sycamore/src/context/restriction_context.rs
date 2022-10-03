use super::*;

#[derive(Debug)]
pub struct RestrictionContext {
    pub presentation: &'static Signal<Presentation>,
    pub opt_sample_id: &'static ReadSignal<Option<SampleId>>,
    restriction_locked_store: Signal<bool>,
}
impl RestrictionContext {
    pub(super) fn new<'a>(scope: Scope<'a>) -> Self {
        let restriction = &create_static_signal(scope, Presentation::default());
        let opt_sample_id = create_static_memo(scope, || restriction.get().opt_sample_id());
        Self {
            presentation: restriction,
            opt_sample_id,
            restriction_locked_store: Default::default(),
        }
    }

    pub(super) fn init(&self, restriction: Presentation) {
        self.restriction_locked_store.set(true);
        self.presentation.set(restriction);
    }

    pub(super) fn opt_sample_id(&self) -> Option<SampleId> {
        self.presentation.get().opt_sample_id()
    }

    pub(super) fn did_lock_restriction(&mut self, restriction: Presentation) {
        self.presentation.set(restriction);
        self.restriction_locked_store.set(true);
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
