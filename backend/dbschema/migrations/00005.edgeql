CREATE MIGRATION m1ow47ozayfoljkqvny5vn7rncivk7kjdtvyyuoe6oojqbde3nd3gq
    ONTO m125c2tzl6ub4ki4a6mttjiw3tqvtaju5x3fe4d4bz3dcj4p2qmloq
{
  ALTER TYPE default::User {
      ALTER PROPERTY auth0_user_id {
          RENAME TO email;
      };
  };
  ALTER TYPE default::User {
      CREATE REQUIRED PROPERTY password_hash: std::str {
          SET REQUIRED USING (<std::str>'temp');
      };
      CREATE REQUIRED PROPERTY username: std::str {
          SET REQUIRED USING (<std::str>'temp');
      };
  };
};
