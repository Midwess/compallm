use crate::app::App;

/// A setup-time capability that installs resources and processors.
pub trait Plugin {
    fn build(&self, app: &mut App);
}
pub trait Plugins<Marker> {
    fn build(self, app: &mut App);
}
pub struct SinglePlugin;
pub struct PluginTuple<M>(std::marker::PhantomData<M>);
impl<P: Plugin> Plugins<SinglePlugin> for P {
    fn build(self, app: &mut App) {
        Plugin::build(&self, app);
    }
}
macro_rules! plugin_tuple {
    ($($plugin:ident : $marker:ident : $index:tt),+) => {
        impl<$($plugin, $marker),+> Plugins<PluginTuple<($($marker,)+)>> for ($($plugin,)+)
        where $($plugin: Plugins<$marker>),+
        { fn build(self, app: &mut App) { $(self.$index.build(app);)+ } }
    };
}
plugin_tuple!(A: MA: 0, B: MB: 1);
plugin_tuple!(A: MA: 0, B: MB: 1, C: MC: 2);
plugin_tuple!(A: MA: 0, B: MB: 1, C: MC: 2, D: MD: 3);
