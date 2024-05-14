CREATE MIGRATION m1tlrxscytpuc2kdkzbe6firevsgpjn4ag2bfkdtkewtimg5db5jrq
    ONTO m1ei7ammrgtd4vuwawgbhjbw3wlshpmb6uxkc7se3guh5cdkvjtvsa
{
  ALTER TYPE default::Plant {
      DROP PROPERTY picture;
  };
};
