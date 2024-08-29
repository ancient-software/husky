use crate::chart::StandardChartDim0;

use super::*;
#[cfg(feature = "egui")]
use egui::Frame;
use husky_control_flow_utils::pass;
use husky_linket_impl::pedestal::JointPedestal;
use husky_standard_linket_impl::pedestal::StandardJointPedestal;
use husky_visual_protocol::{plot::PlotClass, visual::primitive::PrimitiveVisual};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct StandardFigureDim0 {
    joint_pedestal: StandardJointPedestal,
    plots: Vec<StandardPlot>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum StandardPlot {
    Graphics2D {
        images: Vec<ImageVisual>,
        shapes: Vec<ShapeVisual>,
    },
    Graphics3D {
        meshes: (),
    },
    Text,
    Code,
}

/// # constructor
impl StandardFigureDim0 {
    pub(super) fn from_chart(
        (joint_pedestal, composite_visual): StandardChartDim0<CompositeVisual<TraceId>>,
        trace_plot_map: &TracePlotInfos,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        Self::new(
            joint_pedestal,
            composite_visual,
            trace_plot_map,
            visual_synchrotron,
        )
    }

    fn new(
        joint_pedestal: StandardJointPedestal,
        composite_visual: CompositeVisual<TraceId>,
        trace_plot_map: &TracePlotInfos,
        visual_synchrotron: &VisualSynchrotron,
    ) -> Self {
        let mut builder = StandardFigureBuilder::new(
            joint_pedestal,
            composite_visual,
            trace_plot_map,
            visual_synchrotron,
        );
        builder.build_plots();
        builder.finish()
    }
}

/// # builder
struct StandardFigureBuilder<'a> {
    joint_pedestal: StandardJointPedestal,
    composite_visual: CompositeVisual<TraceId>,
    trace_plot_infos: &'a TracePlotInfos,
    visual_synchrotron: &'a VisualSynchrotron,
    plots: Vec<StandardPlot>,
}

impl<'a> StandardFigureBuilder<'a> {
    fn new(
        joint_pedestal: StandardJointPedestal,
        composite_visual: CompositeVisual<TraceId>,
        trace_plot_infos: &'a TracePlotInfos,
        visual_synchrotron: &'a VisualSynchrotron,
    ) -> Self {
        Self {
            joint_pedestal,
            composite_visual,
            trace_plot_infos,
            visual_synchrotron,
            plots: vec![],
        }
    }

    fn build_plots(&mut self) {
        for (traces, plot_class) in &**self.trace_plot_infos {
            let mut plot = match plot_class {
                PlotClass::Void => unreachable!(),
                PlotClass::Graphics2D => StandardPlot::Graphics2D {
                    images: vec![],
                    shapes: vec![],
                },
                PlotClass::Graphics3D => todo!(),
                PlotClass::Any => todo!(),
            };
            for &trace_id in traces {
                self.build_plot(self.composite_visual[trace_id], &mut plot)
            }
            self.plots.push(plot)
        }
    }

    fn build_plot(&mut self, visual: Visual, plot: &mut StandardPlot) {
        match visual {
            Visual::Void => (),
            Visual::Primitive(primitive) => todo!(),
            // self.primitives.push((trace_id, primitive)),
            Visual::Text(_) => todo!(),
            Visual::RichText(_) => todo!(),
            Visual::Image(image) => match plot {
                StandardPlot::Graphics2D { ref mut images, .. } => images.push(image),
                StandardPlot::Graphics3D { meshes } => todo!(),
                StandardPlot::Text => todo!(),
                StandardPlot::Code => todo!(),
            },
            Visual::Shape(shape) => match plot {
                StandardPlot::Graphics2D { ref mut shapes, .. } => shapes.push(shape),
                StandardPlot::Graphics3D { meshes } => todo!(),
                StandardPlot::Text => todo!(),
                StandardPlot::Code => todo!(),
            },
            Visual::Mesh(_) => todo!(),
            Visual::Video(_) => todo!(),
            Visual::Group(group) => {
                for &element in group.elements(self.visual_synchrotron) {
                    self.build_plot(element, plot)
                }
            }
            Visual::Math(_) => todo!(),
        }
    }

    fn finish(self) -> StandardFigureDim0 {
        StandardFigureDim0 {
            joint_pedestal: self.joint_pedestal,
            plots: self.plots,
        }
    }
}

/// # ui
#[cfg(feature = "egui")]
impl StandardFigureDim0 {
    pub(super) fn ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        // ad hoc
        for plot in &self.plots {
            plot.ui(visual_synchrotron, cache, ui)
        }
    }
}

#[cfg(feature = "egui")]
impl StandardPlot {
    pub(super) fn ui(
        &self,
        visual_synchrotron: &VisualSynchrotron,
        cache: &mut FigureUiCache<Ui>,
        ui: &mut Ui,
    ) {
        match self {
            StandardPlot::Graphics2D { images, shapes } => {
                let x = ui.available_width();
                let y = ui.available_height();
                let l = x.min(y);
                let d = (l * 0.9).floor();
                let m = (l - d) * 0.5;
                let (_, rect) = Frame::none()
                    .inner_margin(m)
                    .show(ui, |ui| ui.allocate_space(Vec2::splat(d)))
                    .inner;
                for &image in images {
                    ui.painter().image(
                        cache.texture_id(image, visual_synchrotron, ui),
                        rect,
                        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }
                let mnist_visual_rect = VisualRect::mnist();
                let t = |point: Point| point.to_screen(mnist_visual_rect, rect);
                for shape in shapes {
                    match shape.data(visual_synchrotron) {
                        &ShapeVisualData::LineSegment { start, end, stroke } => {
                            ui.painter().line_segment([t(start), t(end)], stroke);
                        }
                        ShapeVisualData::Contour { points } => {
                            for i in 0..(points.len() - 1) {
                                ui.painter().line_segment(
                                    [t(points[i]), t(points[i + 1])],
                                    (2.0, Color32::WHITE),
                                );
                            }
                        }
                    }
                }
            }
            StandardPlot::Graphics3D { meshes } => todo!(),
            StandardPlot::Text => todo!(),
            StandardPlot::Code => todo!(),
        }
    }
}
