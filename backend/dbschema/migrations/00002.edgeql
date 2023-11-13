CREATE MIGRATION m1c2yp5heuecus34yibakqda3x2n7omwa2cxxlhyg6f2gsvogx75ta
    ONTO m1u26jwh4yjigpie3sno63nqms4bnhyzfmvoepacttskoxjhmcfawa
{
  ALTER TYPE default::User {
      ALTER PROPERTY name {
          DROP CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY username: std::str {
          SET REQUIRED USING (<std::str>{'defaultUsername'});
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
