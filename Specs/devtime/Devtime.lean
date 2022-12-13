import Specs.devtime.Devtime.TraceNode
import Specs.devtime.Trace
import Specs.abstraction.vec_like
import Specs.abstraction.HashMap
import Specs.devtime.protocol

structure DevtimeState where
    presentation: Presentation
    pins: VecSet TraceId

structure TraceFactory where

structure DevtimeDb where
  trace_nodes: List (Option TraceNode)
  opt_active_trace_id: Option TraceId
  figure_canvases: VecSet FigureCanvasKey
  figure_controls: HashMap FigureControlKey FigureControlData
  trace_stalks: HashMap TraceStalkKey TraceStalk
  trace_statss: HashMap TraceStatsKey (Option TraceStats)
  root_traces: List TraceId
  subtrace_ids_map: HashMap SubtracesKey (List TraceId)

structure Devtime where
  state : DevtimeState
  db : DevtimeDb