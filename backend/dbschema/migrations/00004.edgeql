CREATE MIGRATION m125c2tzl6ub4ki4a6mttjiw3tqvtaju5x3fe4d4bz3dcj4p2qmloq
    ONTO m1uxgqgtnmwudtd26lxe6r5rwggwlupbmxok42v2xzdhsx7tcunv3q
{
  ALTER TYPE default::User {
      DROP PROPERTY email;
  };
  ALTER TYPE default::User {
      ALTER PROPERTY password_hash {
          RENAME TO auth0_user_id;
      };
      DROP PROPERTY username;
  };
};
