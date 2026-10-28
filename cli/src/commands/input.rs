use opendap_host_command::CliError;
use opendap_host_command::output::note;
use opendap_host_core::{Need, Outcome, resolve_target};
use opendap_host_transports_emulator::LocalVms;

use super::{Ctx, Doc, emit_plan};
use crate::cli::{InputCmd, InputEvent};

pub fn run(ctx: &Ctx, cmd: InputCmd) -> Result<u8, CliError> {
    let InputCmd::Send { event } = cmd;
    let (vm, target) = resolve_target(&LocalVms, ctx.target.as_ref(), Need::Qmp)?;
    let event = match event {
        InputEvent::Key { role, hold_ms } => opendap_host_model::InputEvent::Key { role, hold_ms },
        InputEvent::KeyDown { role } => opendap_host_model::InputEvent::KeyDown { role },
        InputEvent::KeyUp { role } => opendap_host_model::InputEvent::KeyUp { role },
        InputEvent::Keyboard { key, down, up } => {
            // --down or --up: that half; neither: a press, down now and up as the event sent.
            if !down && !up && !ctx.dry_run {
                use opendap_host_core::InputSink;
                vm.send(&opendap_host_model::InputEvent::Keyboard {
                    key: key.clone(),
                    down: true,
                })?;
            }
            opendap_host_model::InputEvent::Keyboard { key, down }
        }
        InputEvent::Tap { x, y } => opendap_host_model::InputEvent::Tap { x, y },
    };
    let text = match &event {
        opendap_host_model::InputEvent::Key { role, hold_ms } => {
            format!("pressed {} for {hold_ms} ms on {target}", role.as_str())
        }
        opendap_host_model::InputEvent::KeyDown { role } => {
            format!("{} is down on {target}", role.as_str())
        }
        opendap_host_model::InputEvent::KeyUp { role } => {
            format!("released {} on {target}", role.as_str())
        }
        opendap_host_model::InputEvent::Keyboard { key, down } => {
            format!(
                "keyboard {key} {} on {target}",
                if *down { "down" } else { "up" }
            )
        }
        opendap_host_model::InputEvent::Tap { x, y } => format!("tapped {x},{y} on {target}"),
    };
    let sent = match opendap_host_core::input::send(&vm, target, event, ctx.dry_run)? {
        Outcome::Plan(plan) => return emit_plan(ctx, "opendap.input/v1", plan),
        Outcome::Done(sent) => sent,
    };
    ctx.out.emit(
        &Doc {
            schema: "opendap.input/v1",
            body: sent,
        },
        || {
            note(&text);
            String::new()
        },
    );
    Ok(0)
}
