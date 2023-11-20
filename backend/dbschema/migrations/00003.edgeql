CREATE MIGRATION m1uxgqgtnmwudtd26lxe6r5rwggwlupbmxok42v2xzdhsx7tcunv3q
    ONTO m1c2yp5heuecus34yibakqda3x2n7omwa2cxxlhyg6f2gsvogx75ta
{
  CREATE TYPE default::Plant {
      CREATE PROPERTY care_instructions: std::str;
      CREATE REQUIRED PROPERTY description: std::str;
      CREATE REQUIRED PROPERTY name: std::str;
      CREATE REQUIRED PROPERTY picture: std::str;
      CREATE PROPERTY scientific_name: std::str;
      CREATE PROPERTY watering_frequency: std::str;
  };
  ALTER TYPE default::User {
      CREATE MULTI LINK plants: default::Plant;
  };
};
