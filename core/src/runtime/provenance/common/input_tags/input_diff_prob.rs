use crate::common::input_tag::*;

use super::*;

/// An input differentiable probability.
///
/// It contains two elements.
/// The first is an `f64` which represents the probability of the tag.
/// The second is an `Option<T>` which is the original differentiable object.
/// Note that if the second element is provided as `None` then it means we
/// do not treat the object as differentiable and thus we do not need to
/// back-propagate gradients into it.
/// In such case the probability is treated as a constant.
#[derive(Clone)]
pub struct InputDiffProb<T: Clone + 'static>(pub f64, pub Option<T>);

impl<T: Clone + 'static> std::fmt::Debug for InputDiffProb<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl<T: Clone + 'static> From<(f64, Option<T>)> for InputDiffProb<T> {
  fn from((p, t): (f64, Option<T>)) -> Self {
    Self(p, t)
  }
}

impl<T: Clone + 'static> StaticInputTag for InputDiffProb<T> {
  fn from_dynamic_input_tag(t: &DynamicInputTag) -> Option<Self> {
    match t {
      DynamicInputTag::ExclusiveFloat(f, _) => Some(Self(f.clone(), None)),
      DynamicInputTag::Float(f) => Some(Self(f.clone(), None)),
      _ => None,
    }
  }
}

impl<T: Clone + 'static> ConvertFromInputTag<()> for InputDiffProb<T> {
  fn from_input_tag(_: ()) -> Option<Self> {
    None
  }
}

impl<T: Clone + 'static> ConvertFromInputTag<bool> for InputDiffProb<T> {
  fn from_input_tag(b: bool) -> Option<Self> {
    if b {
      None
    } else {
      Some(Self(0.0, None))
    }
  }
}

impl<T: Clone + 'static> ConvertFromInputTag<usize> for InputDiffProb<T> {
  fn from_input_tag(u: usize) -> Option<Self> {
    if u > 0 {
      None
    } else {
      Some(Self(0.0, None))
    }
  }
}

impl<T: Clone + 'static> ConvertFromInputTag<Exclusion> for InputDiffProb<T> {
  fn from_input_tag(_: Exclusion) -> Option<Self> {
    None
  }
}

impl<T: Clone + 'static> ConvertFromInputTag<f64> for InputDiffProb<T> {
  fn from_input_tag(t: f64) -> Option<Self> {
    Some(Self(t, None))
  }
}

impl<T: Clone + 'static> ConvertFromInputTag<InputExclusiveProb> for InputDiffProb<T> {
  fn from_input_tag(t: InputExclusiveProb) -> Option<Self> {
    Some(Self(t.prob, None))
  }
}

impl<T: Clone + 'static> ConvertFromInputTag<InputDiffProb<T>> for InputDiffProb<T> {
  fn from_input_tag(t: InputDiffProb<T>) -> Option<Self> {
    Some(t.clone())
  }
}

impl<T: Clone + 'static> ConvertFromInputTag<InputExclusiveDiffProb<T>> for InputDiffProb<T> {
  fn from_input_tag(t: InputExclusiveDiffProb<T>) -> Option<Self> {
    Some(Self(t.prob, None))
  }
}
