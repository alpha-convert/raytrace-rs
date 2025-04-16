use super::ray::Ray;

pub struct RayPacket {
    rays: Vec<Ray>,
}

impl RayPacket {
    pub fn size(&self) -> usize {
        self.rays.len()
    }
}

impl IntoIterator for RayPacket {
    type Item = Ray;

    type IntoIter = <Vec<Ray> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.rays.into_iter()
    }
}

impl FromIterator<Ray> for RayPacket {
    fn from_iter<T: IntoIterator<Item = Ray>>(iter: T) -> Self {
        RayPacket {
            rays: Vec::from_iter(iter),
        }
    }
}
