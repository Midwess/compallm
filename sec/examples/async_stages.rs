use sec::prelude::*;
use std::{io, num::NonZeroUsize, sync::Arc};

struct Received;
impl StageLabel for Received {}
#[derive(Clone)]
struct Input(String);
struct Output(String);
struct Prefix(String);

async fn map(
    input: Owned<Input>,
    prefix: Shared<Prefix>,
    mut writer: ContextWriter,
) -> io::Result<()> {
    // Await real owned service work here without holding a hecs component borrow.
    tokio::task::yield_now().await;
    writer.insert_resource(Output(format!("{}{}", prefix.0, input.0.0)));
    Ok(())
}
fn report(output: Res<Output>) {
    println!("{}", output.0);
}
struct MappingPlugin;
impl Plugin for MappingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_shared_resource(Arc::new(Prefix("mapped: ".into())));
        app.add_processors(Received, (report.after(map), async_processor(map)));
    }
}
fn main() -> io::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread().build()?;
    runtime.block_on(async {
        let mut app = App::new();
        app.set_max_parallelism(NonZeroUsize::new(2).unwrap());
        app.add_plugins(MappingPlugin);
        let mut execution = app.execution();
        execution
            .resources_mut()
            .insert(Input("hecs-backed SEC".into()));
        execution.run_stage_async(Received).await
    })
}
