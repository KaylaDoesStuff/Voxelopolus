pub struct Player {
    height: i32,
    weight: i32,
    speed: i32,
    name: String
}

pub struct VoxelData {
    pub points: Vec<f32>,
    gravity: bool
}

impl VoxelData {
    pub fn new(points: Vec<f32>, gravity: bool) -> VoxelData {
        VoxelData {
            points,
            gravity,
        }
    }
}

impl Clone for VoxelData {
    fn clone(&self) -> Self {
        VoxelData {
            points: self.points.clone(),
            gravity: self.gravity,
        }
    }
}

impl Player {
    pub fn new(height: i32, weight: i32, speed: i32, name: &str) -> Player {
        Player {
            height,
            weight,
            speed,
            name: name.to_string(),
        }
    }
}

pub static cube_voxel: VoxelData = VoxelData::new(Lazy::new(|| {{vec![
    -0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, 0.5, -0.5,
    -0.5, 0.5, -0.5,
    -0.5, -0.5, 0.5,
    0.5, -0.5, 0.5,
    0.5, 0.5, 0.5,
    -0.5, 0.5, 0.5,
]}; }), false);

pub static index_data_voxel: Lazy<Vec<u32>> = Lazy::new(|| {{vec![
    2, 3, 0, // triangle 2
    4, 5, 6, // triangle 3
    6, 7, 4, // triangle 4
    0, 1, 5, // triangle 5
    5, 4, 0, // triangle 6
    1, 2, 6, // triangle 7
    6, 5, 1, // triangle 8
    2, 3, 7, // triangle 9
    7, 6, 2, // triangle 10
    3, 0, 4, // triangle 11
    4, 7, 3, // triangle 12

]}});
    