use std::cell::RefCell;
use std::rc::Rc;
use deno_core::error::AnyError;
use deno_core::{Extension, op, OpState, RuntimeOptions};
use deno_core::anyhow::anyhow;
use tokio::sync::mpsc::Sender;

pub enum ConsoleMessage {
    Info(String),
    Warn(String),
    Error(String),
    Exit
}

#[op]
async fn op_console_message(state: Rc<RefCell<OpState>>, msg_type: String, msg: String) -> Result<(), AnyError> {
    let mut state_borrow = state.borrow_mut();
    let sender = state_borrow.borrow_mut::<Sender<ConsoleMessage>>();

    let message = match msg_type.as_str() {
        "info" => ConsoleMessage::Info(msg),
        "warn" => ConsoleMessage::Warn(msg),
        "error" => ConsoleMessage::Error(msg),
        _ => return Err(anyhow!("Invalid type!"))
    };

    match sender.send(message).await {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("Failed to send message via mpsc!"))
    }
}

pub async fn run(content: &str, message_sender: Sender<ConsoleMessage>) -> Result<(), AnyError> {
    let runjs_extension = {
        let message_sender = message_sender.clone();

        Extension::builder("runtime")
            .state(move |state| {
                state.put(message_sender.clone());
            })
            .ops(vec![
                op_console_message::decl()
            ])
            .build()
    };

    let mut js_runtime = deno_core::JsRuntime::new(RuntimeOptions {
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    js_runtime.execute_script("[runjs:runtime.js]",  include_str!("./runtime.js"))?;

    js_runtime.execute_script("main", content)?;

    message_sender.send(ConsoleMessage::Exit);

    Ok(())
}