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
        required picture: str;
        scientific_name: str;
        care_instructions: str;
        watering_frequency: str;
    }

    type RefreshToken {
        required token: str;
        required user: User;
        required expires_at: datetime;
        required is_revoked: bool;
    }
}