use crate::plugin::channels::{MethodCallExt, MethodReplyExt};
use mizer_api::handlers::LayoutsHandler;
use mizer_api::models::*;
use mizer_api::RuntimeApi;
use nativeshell::codec::{MethodCall, MethodCallReply, Value};
use nativeshell::shell::{Context, EngineHandle, MethodCallHandler, MethodChannel};
use std::rc::Rc;

#[derive(Clone)]
pub struct LayoutsChannel<R: RuntimeApi> {
    handler: LayoutsHandler<R>,
}

impl<R: RuntimeApi + 'static> MethodCallHandler for LayoutsChannel<R> {
    fn on_method_call(
        &mut self,
        call: MethodCall<Value>,
        resp: MethodCallReply<Value>,
        _: EngineHandle,
    ) {
        log::debug!("mizer.live/layouts -> {}", call.method);
        match call.method.as_str() {
            "getLayouts" => {
                let response = self.get_layouts();

                resp.respond_msg(response);
            }
            "addLayout" => {
                if let Value::String(name) = call.args {
                    let response = self.add_layout(name);

                    resp.respond_msg(response);
                }
            }
            "removeLayout" => {
                if let Value::String(id) = call.args {
                    let response = self.remove_layout(id);

                    resp.respond_msg(response);
                }
            }
            "renameLayout" => {
                let response = call
                    .arguments()
                    .map(|req: RenameLayoutRequest| self.rename_layout(req.id, req.name));

                resp.respond_result(response);
            }
            "removeControl" => {
                if let Err(err) = call.arguments().map(|req| self.remove_control(req)) {
                    resp.respond_error(err);
                } else {
                    resp.send_ok(Value::Null);
                }
            }
            "moveControl" => {
                if let Err(err) = call.arguments().map(|req| self.move_control(req)) {
                    resp.respond_error(err);
                } else {
                    resp.send_ok(Value::Null);
                }
            }
            "renameControl" => {
                if let Err(err) = call.arguments().map(|req| self.rename_control(req)) {
                    resp.respond_error(err);
                } else {
                    resp.send_ok(Value::Null);
                }
            }
            _ => resp.not_implemented(),
        }
    }
}

impl<R: RuntimeApi + 'static> LayoutsChannel<R> {
    pub fn new(handler: LayoutsHandler<R>) -> Self {
        Self { handler }
    }

    pub fn channel(self, context: Rc<Context>) -> MethodChannel {
        MethodChannel::new(context, "mizer.live/layouts", self)
    }

    fn get_layouts(&self) -> Layouts {
        self.handler.get_layouts()
    }

    fn add_layout(&self, name: String) -> Layouts {
        self.handler.add_layout(name)
    }

    fn remove_layout(&self, id: String) -> Layouts {
        self.handler.remove_layout(id)
    }

    fn rename_layout(&self, id: String, name: String) -> Layouts {
        self.handler.rename_layout(id, name)
    }

    fn remove_control(&self, req: RemoveControlRequest) {
        self.handler.remove_control(req.layout_id, req.control_id);
    }

    fn move_control(&self, req: MoveControlRequest) {
        self.handler
            .move_control(req.layout_id, req.control_id, req.position.unwrap());
    }

    fn rename_control(&self, req: RenameControlRequest) {
        self.handler
            .rename_control(req.layout_id, req.control_id, req.name);
    }
}
