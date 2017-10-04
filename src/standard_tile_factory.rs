use tile_factory::TileFactory;
use models::tile::Tile;

pub struct StandardTileFactory;

impl TileFactory for StandardTileFactory {
    fn get_tile_pool() -> Vec<Tile> {
        Vec::new()
    }
}
