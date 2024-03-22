macro_rules! import_all {
    ($($x:ident),+) => {
        $(
            pub mod $x;
            pub use $x::*;
        )*
    };
}

import_all!(
    angle,
    block_states,
    block,
    chunk_section,
    chunk,
    dimension,
    entity_metadata_field,
    entity_metadata,
    gamemode,
    interaction_type,
    player_actions,
    position,
    string,
    uuid,
    varint,
    varlong
);
