use swc_core::ecma::{
    ast::*,
    visit::{VisitMut, VisitMutWith},
};

pub struct HasJSXVisitor {
  has_jsx: bool,
}

impl HasJSXVisitor {
  pub fn test(node: &mut impl VisitMutWith<Self>) -> bool {
      let mut visitor = HasJSXVisitor { has_jsx: false };
      node.visit_mut_children_with(&mut visitor);
      visitor.has_jsx
  }
}

impl VisitMut for HasJSXVisitor {
  fn visit_mut_jsx_element(&mut self, el: &mut JSXElement) {
      el.visit_mut_children_with(self);
      self.has_jsx = true;
  }
}
