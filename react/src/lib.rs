use std::collections::HashMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(u64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}
#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}
struct ComputeCell<'a, T> {
    #[allow(clippy::type_complexity)]
    func: Box<dyn 'a + Fn(&[T]) -> T>,
    dependencies: Vec<CellId>,
    callbacks: HashMap<CallbackId, Box<dyn 'a + FnMut(T)>>,
}

impl<'a, T> ComputeCell<'a, T>
where
    T: Copy + PartialEq + Default,
{
    fn new<F: 'a + Fn(&[T]) -> T>(func: F, dependencies: &[CellId]) -> Self {
        Self {
            func: Box::new(func),
            dependencies: dependencies.to_vec(),
            callbacks: HashMap::new(),
        }
    }

    fn value(&self, reactor: &Reactor<'a, T>) -> T {
        let args = self
            .dependencies
            .iter()
            .map(|&id| reactor.value(id).unwrap())
            .collect::<Vec<_>>();
        (*self.func)(args.as_ref())
    }

    fn add_callback<F: 'a + FnMut(T)>(&mut self, id: CallbackId, callback: F) {
        self.callbacks.insert(id, Box::new(callback));
    }

    fn remove_callback(&mut self, id: CallbackId) -> Result<(), RemoveCallbackError> {
        self.callbacks
            .remove(&id)
            .ok_or(RemoveCallbackError::NonexistentCallback)
            .map(|_| ())
    }

    fn fire_callbacks(&mut self, value: T) {
        for callback in self.callbacks.values_mut() {
            callback(value);
        }
    }
}

#[derive(Default)]
pub struct Reactor<'a, T> {
    current_id: u64,
    input_values: HashMap<InputCellId, T>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<'a, T>>,
}

impl<'a, T> Reactor<'a, T>
where
    T: Copy + PartialEq + Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let input_id = InputCellId(self.next_id());
        self.input_values.insert(input_id, initial);
        input_id
    }

    fn next_id(&mut self) -> u64 {
        self.current_id += 1;
        self.current_id
    }

    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        if let Some(id) = dependencies.iter().find(|id| self.value(**id).is_none()) {
            return Err(*id);
        }
        let compute_id = ComputeCellId(self.next_id());
        self.compute_cells
            .insert(compute_id, ComputeCell::new(compute_func, dependencies));
        Ok(compute_id)
    }

    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(input_id) => self.input_values.get(&input_id).cloned(),
            CellId::Compute(compute_id) => self
                .compute_cells
                .get(&compute_id)
                .map(|cell| cell.value(self)),
        }
    }

    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if !self.input_values.contains_key(&id) {
            return false;
        }
        let old_values = self
            .compute_cells
            .iter()
            .map(|(&compute_id, cell)| (compute_id, cell.value(self)))
            .collect::<Vec<_>>();
        if let Some(x) = self.input_values.get_mut(&id) {
            *x = new_value;
        };
        for (compute_id, old_value) in old_values {
            let cell = self.compute_cells.get(&compute_id).unwrap();
            let new_value = cell.value(self);
            if new_value != old_value {
                let cell = self.compute_cells.get_mut(&compute_id).unwrap();
                cell.fire_callbacks(new_value);
            }
        }
        true
    }

    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let callback_id = CallbackId(self.next_id());
        self.compute_cells
            .get_mut(&id)?
            .add_callback(callback_id, callback);
        Some(callback_id)
    }

    pub fn remove_callback(
        &mut self,
        cell_id: ComputeCellId,
        callback_id: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        self.compute_cells
            .get_mut(&cell_id)
            .ok_or(RemoveCallbackError::NonexistentCell)
            .and_then(|cell| cell.remove_callback(callback_id))
    }
}
