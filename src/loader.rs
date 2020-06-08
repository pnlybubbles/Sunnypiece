use geometry::Triangle;
use material;
use material::Material;
use math::*;
use object::Object;
use std::path::Path;

pub struct Obj {
  models: Vec<tobj::Model>,
  materials: Vec<tobj::Material>,
  material_library: Vec<Box<dyn Material + Sync + Send>>,
}

impl Obj {
  pub fn new(path: &Path) -> Self {
    let (models, materials) = tobj::load_obj(&path, true).expect("ERROR! failed to load models.");
    let material_library = materials
      .iter()
      .map(|v| {
        Box::new(material::Lambertian {
          emittance: v.ambient.into(),
          albedo: v.diffuse.into(),
        }) as Box<dyn Material + Sync + Send>
      })
      .collect::<Vec<_>>();
    Obj {
      models: models,
      materials: materials,
      material_library: material_library,
    }
  }

  pub fn instances(&self) -> Vec<Object> {
    let mut instances: Vec<Object> =
      Vec::with_capacity(self.models.iter().map(|m| m.mesh.indices.len() / 3).sum());
    for m in &self.models {
      for f in 0..m.mesh.indices.len() / 3 {
        let mut polygon = [Vector3::zero(); 3];
        for i in 0..3 {
          let index: usize = f * 3 + i;
          let potition = Vector3::new(
            m.mesh.positions[m.mesh.indices[index] as usize * 3],
            m.mesh.positions[m.mesh.indices[index] as usize * 3 + 1],
            m.mesh.positions[m.mesh.indices[index] as usize * 3 + 2],
          );
          polygon[i] = potition;
        }
        instances.push(Object::new(
          Box::new(Triangle::new(polygon[0], polygon[1], polygon[2])),
          Matrix4::unit(),
          m.mesh
            .material_id
            .map(|id| &self.material_library[id])
            .expect("ERROR! material is not valid."),
        ));
      }
    }
    instances
  }
}
