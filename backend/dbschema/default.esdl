module default {
    type User {
        required property id -> uuid {
            default :=  uuid_generate_v4();
        }
        required property name -> str;
            constraint exclusive;
        required property email -> str {
            constraint exclusive;
        }
        required property password_hash -> str;
    }
}