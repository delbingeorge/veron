use crate::core::isolate::IsolateWrapper;
use crate::utils::{Result, VeronError};
use rusty_v8 as v8;

pub struct Veron {
    isolate: IsolateWrapper,
    context: Option<v8::Global<v8::Context>>,
}

impl Veron {
    pub fn new() -> Self {
        let isolate = IsolateWrapper::new();

        Self {
            isolate,
            context: None,
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        let isolate = self.isolate.get_mut();
        let handle_scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(handle_scope);
        let global = v8::Global::new(handle_scope, context);
        self.context = Some(global);
        Ok(())
    }

    pub fn execute(&mut self, script: &str) -> Result<String> {
        if self.context.is_none() {
            self.initialize()?;
        }

        let isolate = self.isolate.get_mut();
        let handle_scope = &mut v8::HandleScope::new(isolate);
        let context = self.context.as_ref().unwrap();
        let context = v8::Local::new(handle_scope, context.clone());
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        // Compile script
        let code = v8::String::new(scope, script)
            .ok_or_else(|| VeronError::CompilationError("Failed to create V8 string".into()))?;

        let script = v8::Script::compile(scope, code, None)
            .ok_or_else(|| VeronError::CompilationError("Failed to compile script".into()))?;

        // Execute script
        let result = script
            .run(scope)
            .ok_or_else(|| VeronError::ExecutionError("Script returned null".into()))?;

        let result = result
            .to_string(scope)
            .ok_or_else(|| VeronError::ExecutionError("Cannot convert result to string".into()))?;

        Ok(result.to_rust_string_lossy(scope))
    }
}

impl Drop for Veron {
    fn drop(&mut self) {
        self.context = None;
    }
}
