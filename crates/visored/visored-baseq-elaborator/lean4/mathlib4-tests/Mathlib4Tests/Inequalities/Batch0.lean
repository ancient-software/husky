import Mathlib

macro "term_trivial": tactic =>`(tactic|
  first
  | simp; done
  | ring; done
  | ring_nf; done
  | linarith; done
  | fail "Could not prove this goal automatically. Afterall, this is an ad hoc implementation."
)

macro "term_equivalent": tactic =>`(tactic|
  first
  | simp; done
  | ring; done
  | ring_nf; done
  | linarith; done
  | fail "Could not prove this goal automatically. Afterall, this is an ad hoc implementation."
)

namespace Example1
def h := by
  have h2 : 0 = 0 := by term_trivial
  exact ()
end Example1

namespace Example2
def h(x : ℝ) := by
  have h2 : x ^ 2 ≥ 0 := by apply sq_nonneg
  exact ()
end Example2

namespace Example3
def h(x : ℝ)(h1 : x ≥ 1) := by
  have h3 : x - 1 ≥ 0 := by term_equivalent
  exact ()
end Example3

