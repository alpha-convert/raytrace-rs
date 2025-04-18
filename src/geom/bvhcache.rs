use super::bvh::BVH;

//Thread local cache for BVH lookups
pub struct BVHCache<'a, T> {
    bvh: &'a BVH<T>,
    last: &'a BVH<T>,
}
