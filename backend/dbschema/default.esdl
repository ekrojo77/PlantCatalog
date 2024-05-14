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
        required image: bytes;
        scientific_name: str;
        care_instructions: str;
        watering_frequency: str;
    } 
}