use crate::bounding_volume::Aabb;
use crate::math::{Point, Real, Vector};
use crate::shape::Voxels;
use crate::transformation::utils;
use na::{self, RealField};

impl Voxels {
    pub fn to_trimesh(&self) -> (Vec<Point<Real>>, Vec<[u32; 3]>) {
        let aabb =
            Aabb::from_half_extents(Point::origin(), Vector::repeat(self.voxel_size() / 2.0));
        let aabb_vtx = aabb.vertices();

        let mut vtx = vec![];
        let mut idx = vec![];
        for (center, data) in self.centers() {
            let mask = data.free_faces();
            for i in 0..6 {
                if mask & (1 << i) != 0 {
                    let fvid = Aabb::FACES_VERTEX_IDS[i];
                    let base_id = vtx.len() as u32;
                    vtx.push(center + aabb_vtx[fvid.0].coords);
                    vtx.push(center + aabb_vtx[fvid.1].coords);
                    vtx.push(center + aabb_vtx[fvid.2].coords);
                    vtx.push(center + aabb_vtx[fvid.3].coords);

                    if i % 2 == 0 {
                        idx.push([base_id, base_id + 1, base_id + 2]);
                        idx.push([base_id + 0, base_id + 2, base_id + 3]);
                    } else {
                        idx.push([base_id, base_id + 2, base_id + 1]);
                        idx.push([base_id + 0, base_id + 3, base_id + 2]);
                    }
                }
            }
        }

        (vtx, idx)
    }
}
