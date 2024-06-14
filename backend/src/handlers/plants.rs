use crate::models::plants::Plant;

pub async fn add_plant(plant: Plant) -> Result<Plant, String> {
    let client = create_db_client()
        .await
        .map_err(|_| "Failed to get database client".to_string())?;

    let query = r###"
        INSERT Plant {
            name := <str>$0,
            species := <str>$1,
            watering_interval := <int64>$2,
            last_watered := <datetime>$3,
            user := (SELECT User FILTER .username = <str>$4)
        }
    "###;
    client
        .execute(
            query,
            &(
                &plant.name,
                &plant.species,
                &plant.watering_interval,
                &plant.last_watered,
                &plant.user,
            ),
        )
        .await
        .map_err(|e| format!("Failed to add plant to database: {}", e))?;

    Ok(plant)
}
