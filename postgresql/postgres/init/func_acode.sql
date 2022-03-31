CREATE OR REPLACE FUNCTION gen_ticket_code(OUT _tag_id int) AS
$func$
BEGIN
LOOP
   SELECT tag_id
   FROM   tag
   WHERE  tag = _tag
   INTO   _tag_id;

   EXIT WHEN FOUND;

   INSERT INTO tag AS t (tag)
   VALUES (_tag)
   ON     CONFLICT (tag) DO NOTHING
   RETURNING t.tag_id
   INTO   _tag_id;

   EXIT WHEN FOUND;
END LOOP;
END
$func$ LANGUAGE plpgsql;