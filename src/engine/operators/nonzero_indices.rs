use crate::engine::*;

#[derive(Debug)]
pub struct NonzeroIndices<T, U> {
    pub input: BufferRef<T>,
    pub output: BufferRef<U>,
}

impl<'a, T: GenericIntVec<T> + CastUsize, U: GenericIntVec<U>> VecOperator<'a> for NonzeroIndices<T, U> {
    fn execute(&mut self, _: bool, scratchpad: &mut Scratchpad<'a>) -> Result<(), QueryError> {
        let exists = scratchpad.get(self.input);
        let mut unique = scratchpad.get_mut(self.output);
        for (index, &n) in exists.iter().enumerate() {
            if n > T::zero() {
                unique.push(U::from(index).unwrap());
            }
        }
        Ok(())
    }

    fn init(&mut self, _: usize, _: usize, scratchpad: &mut Scratchpad<'a>) {
        scratchpad.set(self.output, Vec::new());
    }

    fn inputs(&self) -> Vec<BufferRef<Any>> { vec![self.input.any()] }
    fn outputs(&self) -> Vec<BufferRef<Any>> { vec![self.output.any()] }
    fn can_stream_input(&self, _: usize) -> bool { true }
    fn can_stream_output(&self, _: usize) -> bool { false }
    fn allocates(&self) -> bool { true }

    fn display_op(&self, _: bool) -> String {
        format!("nonzero_indices({})", self.input)
    }
}

