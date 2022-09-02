/// Perform hardware switch from one context to another.
extern "C" fn _switch(from: &mut Context, to: &Context);

/// Switch from one context to another. Acts as an interface over assembly code.
pub fn switch(from: &mut Context, to: Context) {
    unsafe { _switch(from, &to); }
}
