CREATE MIGRATION m13wjgpji6zb3iulmma6ikperiumhcep7whjw2cowd2rerq7btoyfq
    ONTO m1etnnfbcq6l2mtpmiqvjv6iiardvznsf3oao7spr3ftitpzkxmluq
{
  CREATE TYPE default::Maintenance {
      CREATE REQUIRED LINK plant: default::Plant;
      CREATE PROPERTY fertilization: std::bool;
      CREATE PROPERTY fertilization_type: std::str;
      CREATE PROPERTY image: std::bytes;
      CREATE REQUIRED PROPERTY timestamp: std::datetime;
      CREATE PROPERTY watering: std::bool;
  };
  ALTER TYPE default::Plant {
      CREATE MULTI LINK maintenance_records: default::Maintenance;
      ALTER PROPERTY image {
          RESET OPTIONALITY;
      };
  };
};
