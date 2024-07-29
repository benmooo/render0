use tinytga::RawTga;
use tobj::Model;

pub fn load_model() -> Vec<Model> {
    let obj_file = "assets/african_head.obj";
    // let obj_file = "assets/bugatti.obj";

    let (models, _) = tobj::load_obj(
        &obj_file,
        &tobj::LoadOptions {
            triangulate: true,
            ..Default::default()
        },
    )
    .expect("Failed to OBJ load file");

    models
}

#[allow(unused)]
pub fn load_texture() -> RawTga<'static> {
    let data = include_bytes!("../../assets/african_head_diffuse.tga");
    let img = RawTga::from_slice(data).unwrap();
    img
}

#[cfg(test)]
mod test {
    use super::load_model;

    #[test]
    fn test_load_car() {
        load_model();
    }
}
