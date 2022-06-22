use super::*;

#[derive(Debug, Default)]
pub struct FigureContext {
    figure_canvases: RefCell<HashMap<FigureCanvasKey, &'static FigureCanvasData>>,
    figure_controls: RefCell<HashMap<FigureControlKey, &'static Signal<FigureControlData>>>,
}

impl FigureContext {
    pub(super) fn init(
        &self,
        figure_canvases: HashMap<FigureCanvasKey, &'static FigureCanvasData>,
        figure_controls: HashMap<FigureControlKey, &'static Signal<FigureControlData>>,
    ) {
        *self.figure_canvases.borrow_mut(file!(), line!()) = figure_canvases;
        *self.figure_controls.borrow_mut(file!(), line!()) = figure_controls;
    }

    pub(super) fn set_figure(
        &self,
        scope: Scope<'static>,
        trace: &TraceData,
        attention: &Attention,
        figure: &'static FigureCanvasData,
        figure_control_props: FigureControlData,
    ) {
        assert!(self
            .figure_canvases
            .borrow_mut(file!(), line!())
            .insert(
                FigureCanvasKey::new(trace.kind, trace.id, attention),
                figure
            )
            .is_none());
        self.set_figure_control_data(scope, trace, attention, figure_control_props);
    }

    pub(crate) fn figure_canvas_data(
        &self,
        trace: &TraceData,
        attention: &Attention,
    ) -> &'static FigureCanvasData {
        let figure_canvas_key = FigureCanvasKey::new(trace.kind, trace.id, attention);
        self.figure_canvases.borrow(file!(), line!())[&figure_canvas_key]
    }

    pub(super) fn is_figure_cached(&self, trace: &TraceData, attention: &Attention) -> bool {
        let key = FigureCanvasKey::new(trace.kind, trace.id, attention);
        self.figure_canvases
            .borrow(file!(), line!())
            .contains_key(&key)
    }

    fn set_figure_control_data(
        &self,
        scope: Scope<'static>,
        trace: &TraceData,
        attention: &Attention,
        figure_control_data: FigureControlData,
    ) {
        // this code is buggy because figure_controls borrow mut is not dropped when setting figure control
        // let figure_controls = &mut self.figure_controls.borrow_mut(file!(), line!());
        // let key = FigureControlKey::new(trace, attention);
        // if let Some(figure_control_signal) = figure_controls.get(&key) {
        //     figure_control_signal.set(figure_control_data);
        // } else {
        //     figure_controls.insert(key, Rc::new(Signal::new(figure_control_data)));
        // }
        let opt_figure_control_signal = {
            let figure_controls = &mut self.figure_controls.borrow_mut(file!(), line!());
            let key = FigureControlKey::new(trace.opt_parent_id, trace.kind, trace.id, attention);
            if let Some(figure_control_signal) = figure_controls.get(&key) {
                Some(figure_control_signal.clone())
            } else {
                figure_controls.insert(
                    key,
                    create_static_signal(scope, figure_control_data.clone()),
                );
                None
            }
        };
        opt_figure_control_signal.map(|signal| signal.set(figure_control_data));
    }

    pub(crate) fn figure_control_data(
        &self,
        trace: &TraceData,
        attention: &Attention,
    ) -> &'static Signal<FigureControlData> {
        self.figure_controls.borrow(file!(), line!())
            [&FigureControlKey::new(trace.opt_parent_id, trace.kind, trace.id, attention)]
    }

    // fn update_figure_control_props(
    //     &mut self,
    //     trace_id: TraceId,
    //     updater: Updater<FigureControlData>
    // ) {
    //     let key = self.figure_control_key(trace);
    //     self.figure_control_stores.update(key, updater);
    // }

    // get_figure_control_store(trace: Trace): Readable<FigureControlData> {
    //     let key = self.figure_control_key(trace);
    //     return self.figure_control_stores.get_store(key);
    // }
}
