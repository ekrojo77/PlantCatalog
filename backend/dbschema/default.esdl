module default {
    type User {
        required name: str{
            constraint exclusive;
        }
        required email: str {
            constraint exclusive;
        }
        required password_hash: str;
    }
}