use psl_schema::Schema;

fn load_envs(context: &Schema) -> Result<(), ()> {
    /*
    let env_path = context.root_dir.join(".env");

    if !env_path.is_file() {
        return Ok(());
    }

    for env in dotenv::from_path_iter(&env_path).unwrap() {
        let (name, value) = env.unwrap();

        if std::env::var(&name).is_ok() {
            panic!("Env {name} is already set.");
        }
    }

    dotenv::from_path(&env_path).unwrap();

    Ok(())
    */

    Ok(())
}
