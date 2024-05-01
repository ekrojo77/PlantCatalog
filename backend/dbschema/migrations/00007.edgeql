CREATE MIGRATION m1jvltz7ttqhpwsjw24aeiyh6fsqlsoeetc2m3tnurnhqa4r6jntxa
    ONTO m14clugg53hez5yz6eazhbeizdwh3tl56fcwhsowzjyxb4q4lfeppa
{
  ALTER TYPE default::User {
      ALTER PROPERTY password_hash {
          RENAME TO password;
      };
  };
};
