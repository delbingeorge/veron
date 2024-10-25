use rusty_v8 as v8;
use std::sync::Once;

static V8_INIT: Once = Once::new();

pub struct IsolateWrapper {
    isolate: v8::OwnedIsolate,
}

impl IsolateWrapper {
    pub fn new() -> Self {
        // Initialize V8 platform only once
        V8_INIT.call_once(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });

        let isolate = v8::Isolate::new(v8::CreateParams::default());
        
        Self { isolate }
    }

    pub fn get_mut(&mut self) -> &mut v8::OwnedIsolate {
        &mut self.isolate
    }
}