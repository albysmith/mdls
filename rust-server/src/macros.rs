// $t: storage type for component, $v: struct instance to add to comp, $e: Entity, $w: world
#[macro_export]
macro_rules! add_component {
    ($t:ty, $v:expr, $e:ident, $w:ident) => {
        $w.write_component::<$t>().insert($e, $v)
    };
}
