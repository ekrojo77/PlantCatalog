module default {
    type User {
        required property username -> str {
            constraint exclusive;
        }
        required property password -> str;  # Note: Storing raw passwords is not recommended! This is just for demo purposes.
    };
};