use std::ops::Mul;
mod vec3d;
pub use vec3d::Vec3D;
mod cached_rotation;
use cached_rotation::CachedRotation3D;

/// The `Transform3D` struct is used to manipulate the position of objects in 3D space
#[derive(Debug, Clone, Copy)]
pub struct Transform3D {
    /// The position of the object in 3D space
    pub translation: Vec3D,
    /// The rotation of the object, applied in radians
    pub rotation: Vec3D,
    /// The object's scale
    pub scale: Vec3D,
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Transform3D {
    /// The default transform - no translation, no rotation and 1x scaling
    pub const DEFAULT: Self = Self::new_trs(Vec3D::ZERO, Vec3D::ZERO, Vec3D::ONE);

    /// Create a Transform3D with chosen translation, rotation and scale
    pub const fn new_trs(translation: Vec3D, rotation: Vec3D, scale: Vec3D) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    /// Create a Transform3D with chosen translation and rotation
    pub const fn new_tr(translation: Vec3D, rotation: Vec3D) -> Self {
        Self {
            translation,
            rotation,
            scale: Vec3D::ONE,
        }
    }

    /// Create a Transform3D with chosen translation
    pub const fn new_t(translation: Vec3D) -> Self {
        Self {
            translation,
            rotation: Vec3D::ZERO,
            scale: Vec3D::ONE,
        }
    }

    /// Create a Transform3D with chosen rotation
    pub const fn new_r(rotation: Vec3D) -> Self {
        Self {
            translation: Vec3D::ZERO,
            rotation,
            scale: Vec3D::ONE,
        }
    }

    /// Apply the transform to a slice of vertices
    #[allow(clippy::let_and_return)]
    pub fn apply_to(&self, vertices: &[Vec3D]) -> Vec<Vec3D> {
        let rotation = CachedRotation3D::new(self.rotation);

        vertices
            .iter()
            .map(|v| {
                let rhs = *v;
                let rhs = rhs * self.scale;
                let rhs = rotation.rotate(rhs);
                let rhs = rhs + self.translation;

                rhs
            })
            .collect()
    }

    /// Rotate the given [`Vec3D`] using the `Transform3D`'s rotation field
    pub fn rotate(&self, value: Vec3D) -> Vec3D {
        let rotation = CachedRotation3D::new(self.rotation);

        rotation.rotate(value)
    }
}

impl Mul<Transform3D> for Transform3D {
    type Output = Transform3D;

    fn mul(self, rhs: Transform3D) -> Self::Output {
        Self::new_trs(
            self.translation + rhs.translation,
            self.rotation + rhs.rotation,
            self.scale * rhs.scale,
        )
    }
}

impl Mul<Vec3D> for Transform3D {
    type Output = Vec3D;

    /// Apply the transform to the `Vec3D`
    #[allow(clippy::let_and_return)]
    fn mul(self, rhs: Vec3D) -> Self::Output {
        let rhs = rhs * self.scale;
        let rhs = self.rotate(rhs);
        let rhs = rhs + self.translation;

        rhs
    }
}