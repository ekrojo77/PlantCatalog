CREATE MIGRATION m1etnnfbcq6l2mtpmiqvjv6iiardvznsf3oao7spr3ftitpzkxmluq
    ONTO m1tlrxscytpuc2kdkzbe6firevsgpjn4ag2bfkdtkewtimg5db5jrq
{
  ALTER TYPE default::Plant {
      CREATE REQUIRED PROPERTY image: std::bytes {
          SET REQUIRED USING (<std::bytes>{});
      };
  };
};
