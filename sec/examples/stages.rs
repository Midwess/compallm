use sec::prelude::*;
use std::{io, sync::Arc};

struct Received;
impl StageLabel for Received {}
struct Input(String);
struct Output(String);
struct Prefix(String);

fn validate(input: Res<Input>) -> io::Result<()> {
    if input.0.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "empty input"));
    }
    Ok(())
}
fn map(input: Res<Input>, prefix: Shared<Prefix>, mut writer: ContextWriter) {
    writer.insert_resource(Output(format!("{}{}", prefix.0, input.0)));
}
fn report(output: Res<Output>, mut runs: Local<usize>) {
    *runs += 1;
    println!("run {}: {}", *runs, output.0);
}
struct MappingPlugin;
impl Plugin for MappingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_shared_resource(Arc::new(Prefix("mapped: ".into())));
        app.add_processors(Received, (report.after(map), map.after(validate), validate));
    }
}
fn main() -> io::Result<()> {
    let mut app = App::new();
    app.add_plugins(MappingPlugin);
    let mut execution = app.execution();
    execution
        .resources_mut()
        .insert(Input("first input".into()));
    execution.run_stage(Received)?;
    execution
        .resources_mut()
        .insert(Input("same stage, new input".into()));
    execution.run_stage(Received)?;
    Ok(())
}
