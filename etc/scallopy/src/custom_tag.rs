use pyo3::{Py, PyAny, Python};
use scallop_core::runtime::dynamic;
use scallop_core::runtime::provenance;

#[derive(Clone, Debug)]
pub struct CustomTag(pub Py<PyAny>);

impl CustomTag {
  pub fn new(tag: Py<PyAny>) -> Self {
    Self(tag)
  }
}

impl std::fmt::Display for CustomTag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self, f)
  }
}

impl provenance::Tag for CustomTag {
  type Context = CustomTagContext;
}

#[derive(Clone, Debug)]
pub struct CustomTagContext(pub Py<PyAny>);

impl provenance::ProvenanceContext for CustomTagContext {
  type Tag = CustomTag;

  type InputTag = Py<PyAny>;

  type OutputTag = Py<PyAny>;

  fn name() -> &'static str {
    "scallopy-custom"
  }

  fn tagging_fn(&mut self, i: Self::InputTag) -> Self::Tag {
    Python::with_gil(|py| {
      let result = self.0.call_method(py, "tagging_fn", (i,), None).unwrap();
      Self::Tag::new(result)
    })
  }

  fn tagging_disjunction_fn(&mut self, i: Vec<Self::InputTag>) -> Vec<Self::Tag> {
    Python::with_gil(|py| {
      let result: Vec<Py<PyAny>> = self
        .0
        .call_method(py, "tagging_disjunction_fn", (i,), None)
        .unwrap()
        .extract(py)
        .unwrap();
      result
        .into_iter()
        .map(|t| {
          let t: Py<PyAny> = t.into();
          Self::Tag::new(t)
        })
        .collect()
    })
  }

  fn recover_fn(&self, t: &Self::Tag) -> Self::OutputTag {
    Python::with_gil(|py| {
      self
        .0
        .call_method(py, "recover_fn", (t.0.clone(),), None)
        .unwrap()
        .extract(py)
        .unwrap()
    })
  }

  fn discard(&self, t: &Self::Tag) -> bool {
    Python::with_gil(|py| {
      self
        .0
        .call_method(py, "discard", (t.0.clone(),), None)
        .unwrap()
        .extract(py)
        .unwrap()
    })
  }

  fn zero(&self) -> Self::Tag {
    Python::with_gil(|py| {
      Self::Tag::new(
        self
          .0
          .call_method(py, "zero", (), None)
          .unwrap()
          .extract(py)
          .unwrap(),
      )
    })
  }

  fn one(&self) -> Self::Tag {
    Python::with_gil(|py| {
      Self::Tag::new(
        self
          .0
          .call_method(py, "one", (), None)
          .unwrap()
          .extract(py)
          .unwrap(),
      )
    })
  }

  fn add(&self, t1: &Self::Tag, t2: &Self::Tag) -> Self::Tag {
    Python::with_gil(|py| {
      let input = (t1.0.clone(), t2.0.clone());
      Self::Tag::new(
        self
          .0
          .call_method(py, "add", input, None)
          .unwrap()
          .extract(py)
          .unwrap(),
      )
    })
  }

  fn mult(&self, t1: &Self::Tag, t2: &Self::Tag) -> Self::Tag {
    Python::with_gil(|py| {
      let input = (t1.0.clone(), t2.0.clone());
      Self::Tag::new(
        self
          .0
          .call_method(py, "mult", input, None)
          .unwrap()
          .extract(py)
          .unwrap(),
      )
    })
  }

  fn dynamic_aggregate<'a>(
    &self,
    _: &dynamic::DynamicAggregateOp,
    _: dynamic::DynamicElements<Self::Tag>,
  ) -> Vec<scallop_core::runtime::dynamic::DynamicElement<Self::Tag>> {
    unimplemented!()
    // match op {
    //   dynamic::DynamicAggregateOp::Count(expr) => Python::with_gil(|py| {
    //     let projected_batch = dynamic::DynamicAggregateOp::project_batch_helper(batch, expr, self);
    //     let vectorized_batch = projected_batch
    //       .into_iter()
    //       .map(|e| (e.tag.0.clone(), to_python_tuple(&e.tuple)))
    //       .collect::<Vec<_>>();
    //     let result: Vec<(Py<PyAny>, Py<PyAny>)> = self
    //       .0
    //       .call_method(py, "aggregate_count", (vectorized_batch,), None)
    //       .unwrap()
    //       .extract(py)
    //       .unwrap();
    //     result
    //       .into_iter()
    //       .map(|(tag, tuple)| {
    //         let tuple = from_python_tuple(
    //           tuple.as_ref(py),
    //           &<TupleType as FromType<usize>>::from_type(),
    //         )
    //         .unwrap();
    //         dynamic::DynamicElement::new(tuple, Self::Tag::new(tag))
    //       })
    //       .collect()
    //   }),
    //   dynamic::DynamicAggregateOp::Exists => Python::with_gil(|py| {
    //     let vectorized_batch = batch
    //       .map(|e| (e.tag.0.clone(), to_python_tuple(&e.tuple)))
    //       .collect::<Vec<_>>();
    //     let result: Vec<(Py<PyAny>, Py<PyAny>)> = self
    //       .0
    //       .call_method(py, "aggregate_exists", (vectorized_batch,), None)
    //       .unwrap()
    //       .extract(py)
    //       .unwrap();
    //     result
    //       .into_iter()
    //       .map(|(tag, tuple)| {
    //         let tuple = from_python_tuple(
    //           tuple.as_ref(py),
    //           &<TupleType as FromType<usize>>::from_type(),
    //         )
    //         .unwrap();
    //         dynamic::DynamicElement::new(tuple, Self::Tag::new(tag))
    //       })
    //       .collect()
    //   }),
    //   dynamic::DynamicAggregateOp::Unique(expr) => Python::with_gil(|py| {
    //     let projected_batch = dynamic::DynamicAggregateOp::project_batch_helper(batch, expr, self);
    //     let ty = projected_batch.get(0).map(|elem| elem.tuple.tuple_type());
    //     if let Some(ty) = ty {
    //       let vectorized_batch = projected_batch
    //         .into_iter()
    //         .map(|e| (e.tag.0.clone(), to_python_tuple(&e.tuple)))
    //         .collect::<Vec<_>>();
    //       let result: Vec<(Py<PyAny>, Py<PyAny>)> = self
    //         .0
    //         .call_method(py, "aggregate_unique", (vectorized_batch,), None)
    //         .unwrap()
    //         .extract(py)
    //         .unwrap();
    //       result
    //         .into_iter()
    //         .map(|(tag, tuple)| {
    //           let tuple = from_python_tuple(tuple.as_ref(py), &ty).unwrap();
    //           dynamic::DynamicElement::new(tuple, Self::Tag::new(tag))
    //         })
    //         .collect()
    //     } else {
    //       vec![]
    //     }
    //   }),
    //   _ => unimplemented!(),
    // }
  }
}
