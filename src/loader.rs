use geometry::Triangle;
use geometry::UUID;
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
        let emittance = v
          .unknown_param
          .get("Ke")
          .and_then(|s| Obj::parse_vector(s))
          .unwrap_or(Vector3::zero());
        let roughness = v.unknown_param.get("Pr").and_then(|s| Obj::parse_float(s));
        let albedo = v.diffuse[..].into();
        match roughness {
          Some(r) => Box::new(material::GGX {
            roughness: r,
            reflectance: albedo,
          }) as Box<dyn Material + Sync + Send>,
          None => Box::new(material::Lambertian {
            emittance: emittance,
            albedo: albedo,
          }),
        }
      })
      .collect::<Vec<_>>();
    Obj {
      models: models,
      materials: materials,
      material_library: material_library,
    }
  }

  fn parse_float(input: &String) -> Option<f32> {
    input.trim().parse::<f32>().ok()
  }

  fn parse_vector(input: &String) -> Option<Vector3> {
    let parsed = input
      .split_ascii_whitespace()
      .flat_map(|v| v.trim().parse::<f32>())
      .collect::<Vec<_>>();
    if parsed.len() >= 3 {
      Some(parsed[..].into())
    } else {
      None
    }
  }

  pub fn instances<'a>(
    &'a self,
    fallback_material: &'a Box<dyn Material + Send + Sync>,
    uuid: &mut UUID,
  ) -> Vec<Object> {
    let mut instances: Vec<Object> =
      Vec::with_capacity(self.models.iter().map(|m| m.mesh.indices.len() / 3).sum());
    for m in &self.models {
      for f in 0..m.mesh.indices.len() / 3 {
        let mut coord = [Vector3::zero(); 3];
        let mut normal = [Vector3::zero(); 3];
        for i in 0..3 {
          let index: usize = f * 3 + i;
          let a = m.mesh.indices[index] as usize * 3;
          coord[i] = Vector3::new(
            m.mesh.positions[a],
            m.mesh.positions[a + 1],
            m.mesh.positions[a + 2],
          );
          normal[i] = Vector3::new(
            m.mesh.normals[a],
            m.mesh.normals[a + 1],
            m.mesh.normals[a + 2],
          );
        }
        instances.push(Object::new(
          Box::new(Triangle::new(
            coord[0], coord[1], coord[2], normal[0], normal[1], normal[2], uuid,
          )),
          Matrix4::unit(),
          m.mesh
            .material_id
            .map(|id| &self.material_library[id])
            .unwrap_or(fallback_material),
        ));
      }
    }
    instances
  }
}
