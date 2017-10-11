use models::tile::Tile;
use models::feature::Feature;
use models::direction::Direction;

pub struct TileBuilder {
    tile: Tile
}

impl TileBuilder {
    pub fn new() -> Self {
        TileBuilder {
            tile: Tile::new()
        }
    }

    pub fn with_road(&mut self, connections: &[Direction]) -> &mut Self {
        self.tile.features.push(Feature::Road {
            connections: connections.iter().cloned().collect()
        });

        self
    }

    pub fn with_field(&mut self, connections: &[Direction]) -> &mut Self {
        self.tile.features.push(Feature::Field {
            connections: connections.iter().cloned().collect()
        });

        self
    }

    pub fn build(&self) -> Tile {
        self.tile.clone()
    }
}
