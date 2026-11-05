use crate::{
    resources::Resources,
    stage::{compiled::CompiledStage, graph::StageGraph},
};
use std::io;

pub(in crate::stage) fn run(
    graph: &mut StageGraph,
    compiled: &CompiledStage,
    resources: &mut Resources,
) -> io::Result<()> {
    for &index in &compiled.order {
        let config = graph.nodes[index].as_mut().expect("available processor");
        let result = (|| {
            if !config.allowed(resources)? {
                return Ok(false);
            }
            config.processor.run(resources)?;
            Ok(true)
        })();
        match result {
            Ok(true) => resources.apply_pending(),
            Ok(false) => resources.discard_pending(),
            Err(error) => {
                resources.discard_pending();
                return Err(error);
            }
        }
    }
    Ok(())
}
