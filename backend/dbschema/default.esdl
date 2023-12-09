module default {
    type User {
        required auth0_user_id: str;  # Store Auth0 user identifier
        required name: str;
        multi plants: Plant;
    }

    type Plant {
        required name: str;
        required description: str;
        required picture: str;
        scientific_name: str;
        care_instructions: str;
        watering_frequency: str;
    }
}