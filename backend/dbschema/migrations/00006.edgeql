CREATE MIGRATION m14clugg53hez5yz6eazhbeizdwh3tl56fcwhsowzjyxb4q4lfeppa
    ONTO m1ow47ozayfoljkqvny5vn7rncivk7kjdtvyyuoe6oojqbde3nd3gq
{
  CREATE TYPE default::RefreshToken {
      CREATE REQUIRED LINK user: default::User;
      CREATE REQUIRED PROPERTY expires_at: std::datetime;
      CREATE REQUIRED PROPERTY is_revoked: std::bool;
      CREATE REQUIRED PROPERTY token: std::str;
  };
  ALTER TYPE default::User {
      DROP PROPERTY email;
  };
};
