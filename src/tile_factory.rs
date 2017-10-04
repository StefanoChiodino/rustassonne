use models::tile::Tile;

pub trait TileFactory {
    fn get_tile_pool() -> Vec<Tile>;
}
