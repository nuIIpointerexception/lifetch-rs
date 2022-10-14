use crate::modules::module::FetchModule;

pub mod module;

pub struct FetchModules {
    pub modules: Vec<FetchModule>,
}

impl FetchModules {
    #[inline]
    pub fn new() -> FetchModules {
        FetchModules { modules: vec![] }
    }

    #[inline]
    /// This function adds a new FetchModule to the list of modules
    ///
    /// # Arguments:
    /// * `module`: The module to add to the list of modules.
    ///
    pub fn add_module(&mut self, module: FetchModule) {
        self.modules.push(module);
    }

    #[inline]
    /// Gets all modules in the list of modules.
    ///
    /// # Returns:
    /// The list of modules.
    pub fn get_modules(&self) -> &Vec<FetchModule> {
        &self.modules
    }

    #[inline]
    #[allow(dead_code)]
    /// It returns the number of modules in the module list.
    ///
    /// # Returns:
    /// The number of modules in the module list
    pub fn get_module_count(&self) -> usize {
        self.modules.len()
    }
}
