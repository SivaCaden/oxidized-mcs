

pub struct Vector2D {
    x: f32,
    y: f32,
}
impl Vector2D {
    pub fn new() -> Vector2D {
        Vector2D{x: 0.0, y: 0.0}
    }

    pub fn add(self, other: Vector2D) -> Vector2D{
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn sub(self, other: Vector2D) -> Vector2D{
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    pub fn mul(self, other: Vector2D) -> Vector2D{
        Vector2D {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
    pub fn div(self, other: Vector2D) -> Vector2D{
        Vector2D {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
    pub fn floor_div(self, other: Vector2D) -> Vector2D{
        Vector2D {
            x: (self.x / other.x).floor(),
            y: (self.y / other.y).floor(),
        }
    }

    pub fn eq(self, other: Vector2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}


pub struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}
impl Vector3D {
    pub fn new() -> Vector3D {
        Vector3D{x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn add(self, other: Vector3D) -> Vector3D{
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    pub fn sub(self, other: Vector3D) -> Vector3D{
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    pub fn mul(self, other: Vector3D) -> Vector3D{
        Vector3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
    pub fn div(self, other: Vector3D) -> Vector3D{
        Vector3D {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
    pub fn floor_div(self, other: Vector3D) -> Vector3D{
        Vector3D {
            x: (self.x / other.x).floor(),
            y: (self.y / other.y).floor(),
            z: (self.z / other.z).floor(),
        }
    }

    pub fn eq(self, other: Vector3D) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}