use alloc::format;
use alloc::string::{String, ToString};

use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::Transform2D;
use kernel_lib::layers::layer_key::LayerKey;
use kernel_lib::layers::terminal::TerminalLayer;
use kernel_lib::layers::text::command::{Command, CommandAction, CommandArgs, CommandResult};
use kernel_lib::layers::text::config;
use kernel_lib::task;

use pci::pci_device_searcher::PciDeviceSearcher;

use crate::layers::TERMINAL_LAYER_KEY;

pub(crate) fn terminal() -> LayerKey {
    let pos = Vector2D::new(100, 200);
    let size = Size::new(500, 16 * 20 + 10 + 17);
    let transform = Transform2D::new(pos, size);
    let config = config::Builder::terminal()
        .add_command(Command::new("echo", echo))
        .add_command(Command::new("clear", clear))
        .add_command(Command::new("lspci", lspci))
        .add_command(Command::new("sleep", sleep))
        .add_command(Command::new("wakeup", wakeup))
        .build();

    TerminalLayer::new(transform, config)
        .into_enum()
        .into_layer_key(TERMINAL_LAYER_KEY)
}


fn echo(args: CommandArgs) -> CommandResult {
    if args.is_empty() {
        return Err("Not Command Args".to_string());
    }

    Ok(CommandAction::Output(args[0].to_string()))
}


fn clear(_args: CommandArgs) -> CommandResult {
    Ok(CommandAction::Clear)
}


fn lspci(_args: CommandArgs) -> CommandResult {
    if let Some(devices) = PciDeviceSearcher::new()
        .searches()
        .filter(|device| !device.is_empty())
    {
        let mut output: String = devices
            .iter()
            .map(|d| format!("{:?}\n", d))
            .collect::<String>();

        output.pop();

        Ok(CommandAction::Output(output))
    } else {
        Err("Not found Pci Devices".to_string())
    }
}


fn sleep(args: CommandArgs) -> CommandResult {
    let task_id = parse_task_id(args)?;

    task::sleep(task_id).map_err(|e| format!("{e:?}"))?;

    Ok(CommandAction::output(format!("sleep to {task_id}")))
}


fn wakeup(args: CommandArgs) -> CommandResult {
    let task_id = parse_task_id(args)?;

    task::wakeup(task_id).map_err(|e| format!("{e:?}"))?;

    Ok(CommandAction::output(format!("wakeup to {task_id}")))
}


fn parse_task_id(args: CommandArgs) -> Result<u64, String> {
    args.first()
        .and_then(|task_id| task_id.parse::<u64>().ok())
        .ok_or("Must be specify correct task id".to_string())
}
