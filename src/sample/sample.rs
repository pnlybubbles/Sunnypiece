use super::pdf::Measure;

pub struct Sample<T, P>
  where P: Measure
{
  pub value: T,
  pub pdf: P,
}
