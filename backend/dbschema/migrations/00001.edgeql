CREATE MIGRATION m1u26jwh4yjigpie3sno63nqms4bnhyzfmvoepacttskoxjhmcfawa
    ONTO initial
{
  CREATE TYPE default::User {
      CREATE REQUIRED PROPERTY email: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY name: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY password_hash: std::str;
  };
};
