module default {
    type User {
        required name: str;
        required username: str {
            constraint exclusive;
        }
        required email: str {
            constraint exclusive;
        }
        required password_hash: str;
    }
}