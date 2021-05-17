/*
 */
use crate::{
    cgmath::{aabb::AxisAlignedBoundingBox, ray::Ray, transform::Transform},
    raytracer::{
        material::{Material, MaterialDef},
        shape::{HittableShape, ShapeDef, ShapeHit},
    },
};

/*
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ShapeId(usize);

/*
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MaterialId(usize);

/*
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TransformId(usize);

/*
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ObjectId(usize);

/*
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AabbId(usize);

/*
*/
#[derive(Debug, Clone)]
struct Object {
    shape: ShapeId,
    material: MaterialId,
    transform: TransformId,
    aabb: AabbId,
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub object: ObjectId,
    pub shape_hit: ShapeHit,
}

pub struct Scene {
    shapes: Vec<ShapeDef>,
    materials: Vec<MaterialDef>,
    transforms: Vec<Transform>,
    aabbs: Vec<AxisAlignedBoundingBox>,
    objects: Vec<Object>,
}

impl Scene {
    fn new(
        shapes: Vec<ShapeDef>,
        materials: Vec<MaterialDef>,
        transforms: Vec<Transform>,
        aabbs: Vec<AxisAlignedBoundingBox>,
        objects: Vec<Object>,
    ) -> Scene {
        // TODO: construct BVH here
        Scene {
            shapes,
            materials,
            transforms,
            aabbs,
            objects,
        }
    }

    pub fn get_shape(
        &self,
        object: ObjectId,
    ) -> &dyn HittableShape {
        assert!(object.0 < self.objects.len());

        let object = &self.objects[object.0];

        &self.shapes[object.shape.0]
    }

    pub fn get_material(
        &self,
        object: ObjectId,
    ) -> &dyn Material {
        assert!(object.0 < self.objects.len());

        let object = &self.objects[object.0];

        &self.materials[object.material.0]
    }

    pub fn nearest_hit(
        &self,
        ray: &Ray,
        near: f32,
        far: f32,
    ) -> Option<Hit> {
        // horrible linear brute force algorithm
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct SceneBuilder {
    shapes: Vec<ShapeDef>,
    materials: Vec<MaterialDef>,
    transforms: Vec<Transform>,
    aabbs: Vec<AxisAlignedBoundingBox>,
    objects: Vec<Object>,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            shapes: Vec::new(),
            materials: Vec::new(),
            transforms: Vec::new(),
            aabbs: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn build(self) -> Scene {
        let SceneBuilder {
            shapes,
            materials,
            transforms,
            aabbs,
            objects,
        } = self;
        Scene::new(shapes, materials, transforms, aabbs, objects)
    }

    pub fn add_shape(
        &mut self,
        shape: ShapeDef,
    ) -> ShapeId {
        self.shapes.push(shape);
        ShapeId(self.shapes.len() - 1)
    }

    pub fn add_material(
        &mut self,
        material: MaterialDef,
    ) -> MaterialId {
        self.materials.push(material);
        MaterialId(self.materials.len() - 1)
    }

    pub fn add_object(
        &mut self,
        shape_id: ShapeId,
        material_id: MaterialId,
        transform: Transform,
    ) -> ObjectId {
        assert!(shape_id.0 < self.shapes.len());
        assert!(material_id.0 < self.materials.len());

        //
        let aabb = self.shapes[shape_id.0].aabb().apply_transform(&transform);
        self.aabbs.push(aabb);
        let aabb_id = AabbId(self.aabbs.len() - 1);

        //
        self.transforms.push(transform);
        let transform_id = TransformId(self.transforms.len() - 1);

        //
        self.objects.push(Object {
            shape: shape_id,
            material: material_id,
            transform: transform_id,
            aabb: aabb_id,
        });

        ObjectId(self.objects.len() - 1)
    }
}
