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
#[cfg(test)]
mod test {
    use super::load_model;

    #[test]
    fn test_load_car() {
        load_model();
    }
}
