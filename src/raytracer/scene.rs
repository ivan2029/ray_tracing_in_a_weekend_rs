use crate::raytracer::material::*;
use crate::raytracer::ray::*;

//
//
//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShapeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectId(usize);

//
//
//

#[derive(Debug, Clone)]
struct Object {
    shape: ShapeId,
    material: MaterialId,
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub object: ObjectId,
    pub shape_hit: ShapeHit,
}

//
//
//

#[derive(Debug)]
pub struct Scene {
    shapes: Vec<Box<dyn HittableShape>>,
    materials: Vec<Box<dyn Material>>,
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            shapes: Vec::new(),
            materials: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn insert_shape<S>(
        &mut self,
        shape: S,
    ) -> ShapeId
    where
        S: 'static + HittableShape,
    {
        self.shapes.push(Box::new(shape));
        ShapeId(self.shapes.len() - 1)
    }

    pub fn insert_material<M>(
        &mut self,
        material: M,
    ) -> MaterialId
    where
        M: 'static + Material,
    {
        self.materials.push(Box::new(material));
        MaterialId(self.materials.len() - 1)
    }

    pub fn insert_object(
        &mut self,
        shape: ShapeId,
        material: MaterialId,
    ) -> ObjectId {
        assert!(shape.0 < self.shapes.len());
        assert!(material.0 < self.materials.len());

        self.objects.push(Object { shape, material });

        ObjectId(self.shapes.len() - 1)
    }

    pub fn get_shape(
        &self,
        object: ObjectId,
    ) -> &dyn HittableShape {
        assert!(object.0 < self.objects.len());

        let object = &self.objects[object.0];

        self.shapes[object.shape.0].as_ref()
    }

    pub fn get_material(
        &self,
        object: ObjectId,
    ) -> &dyn Material {
        assert!(object.0 < self.objects.len());

        let object = &self.objects[object.0];

        self.materials[object.material.0].as_ref()
    }

    pub fn nearest_hit(
        &self,
        ray: &Ray,
        near: f32,
        far: f32,
    ) -> Option<Hit> {
        // horrible linear brute force algorithm
        let inf_hit = Hit {
            object: ObjectId(usize::MAX),
            shape_hit: ShapeHit {
                t: f32::INFINITY,
                ..Default::default()
            },
        };

        fn choose_nearer(
            a: Hit,
            b: Hit,
        ) -> Hit {
            if a.shape_hit.t < b.shape_hit.t {
                a
            } else {
                b
            }
        }

        let hit = (0..self.objects.len())
            .filter_map(|object| {
                let object = ObjectId(object);
                let shape = self.get_shape(object);
                shape
                    .hit(ray, near, far)
                    .map(|shape_hit| Hit { object, shape_hit })
            })
            .fold(inf_hit, choose_nearer);

        if f32::is_infinite(hit.shape_hit.t) {
            None
        } else {
            Some(hit)
        }
    }
}
