use crate::cgmath::{point3::Point3, ray::Ray, transform::Transform};
use std::ops::Range;

#[derive(Debug, Clone, Default)]
pub struct AxisAlignedBoundingBox {
    x: (f32, f32),
    y: (f32, f32),
    z: (f32, f32),
}

impl AxisAlignedBoundingBox {
    pub fn new(
        x_range: Range<f32>,
        y_range: Range<f32>,
        z_range: Range<f32>,
    ) -> AxisAlignedBoundingBox {
        assert!(x_range.start < x_range.end);
        assert!(y_range.start < y_range.end);
        assert!(z_range.start < z_range.end);

        AxisAlignedBoundingBox {
            x: (x_range.start, x_range.end),
            y: (y_range.start, y_range.end),
            z: (z_range.start, z_range.end),
        }
    }

    pub fn apply_transform(
        &self,
        t: &Transform,
    ) -> AxisAlignedBoundingBox {
        let mut box_points = [
            // z min plane
            Point3::new(self.x.0, self.y.0, self.z.0),
            Point3::new(self.x.0, self.y.1, self.z.0),
            Point3::new(self.x.1, self.y.0, self.z.0),
            Point3::new(self.x.1, self.y.1, self.z.0),
            // z max plane
            Point3::new(self.x.0, self.y.0, self.z.1),
            Point3::new(self.x.0, self.y.1, self.z.1),
            Point3::new(self.x.1, self.y.0, self.z.1),
            Point3::new(self.x.1, self.y.1, self.z.1),
        ];

        for p in &mut box_points {
            *p = p.apply_transform(t);
        }

        let mut x_min = box_points[0].x();
        let mut x_max = box_points[0].x();
        let mut y_min = box_points[0].y();
        let mut y_max = box_points[0].y();
        let mut z_min = box_points[0].z();
        let mut z_max = box_points[0].z();

        for p in box_points.iter().skip(1) {
            x_min = x_min.min(p.x());
            x_max = x_max.max(p.x());
            y_min = y_min.min(p.y());
            y_max = y_max.max(p.y());
            z_min = z_min.min(p.z());
            z_max = z_max.max(p.z());
        }

        AxisAlignedBoundingBox::new(x_min..x_max, y_min..y_max, z_min..z_max)
    }

    fn is_hit(
        &self,
        ray: &Ray,
    ) -> bool {
        todo!()
    }
}
