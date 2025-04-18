use super::aabb::AABB;

pub trait Bbox {
    fn bbox(&self) -> AABB;
}
