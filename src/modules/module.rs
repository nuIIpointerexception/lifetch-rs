#[derive(PartialEq, Eq)]
pub struct FetchModule {
    pub string: String,
}

impl FetchModule {
    #[inline]
    /// This function creates a new FetchModule.
    ///
    /// # Arguments:
    /// * `String`: The String to add to the module.
    ///
    /// # Returns:
    /// The new FetchModule.
    ///
    /// # Example:
    /// ```
    /// use fetch::module::FetchModule;
    ///
    /// let module = FetchModule::new("test".to_string());
    /// ```
    ///
    pub fn new(string: String) -> FetchModule {
        FetchModule { string }
    }
    #[inline]
    /// This function returns the String of the module.
    /// # Returns:
    /// The String of the module.
    /// # Example:
    /// ```
    /// use fetch::module::FetchModule;
    /// let module = FetchModule::new("test".to_string());
    /// assert_eq!(module.get_string(), "test");
    /// ```
    ///
    pub fn get_string(&self) -> &String {
        &self.string
    }
}
