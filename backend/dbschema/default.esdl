module default {
    type User {
        required name: str;
        required username: str;
        required password: str;
        multi plants: Plant;
    }

    type Plant {
        required name: str;
        required description: str;
        image: bytes;
        scientific_name: str;
        care_instructions: str;
        watering_frequency: str;
        multi maintenance_records: Maintenance;
    } 

    type Maintenance {
        required timestamp: datetime;
        watering: bool;
        fertilization: bool;
        fertilization_type: str;
        image: bytes;
        required plant: Plant;
    }
}