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
    chunk,
    chunk_section,
    entity_metadata_field,
    entity_metadata,
    gamemode,
    palette,
    paletted_container,
    player_actions,
    position,
    string,
    uuid,
    varint,
    varlong
);
