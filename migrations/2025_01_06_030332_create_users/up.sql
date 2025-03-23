CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  friends JSONB NOT NULL DEFAULT '[]'::jsonb,
  rank TEXT NOT NULL DEFAULT 'user',
  registered_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TYPE RELATIONSHIP as ENUM('pending', 'accepted');

CREATE TABLE friendships (
  user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  friend_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  status RELATIONSHIP NOT NULL DEFAULT 'pending',
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  PRIMARY KEY (user_id, friend_id)
);

CREATE OR REPLACE FUNCTION update_friends_list() RETURNS TRIGGER AS $$
BEGIN
  IF NEW.status = 'accepted' THEN
    -- Добавляем друга в массив, если статус стал "accepted"
    UPDATE users
    SET friends = (SELECT jsonb_agg(elem)
                   FROM (SELECT DISTINCT jsonb_array_elements_text(COALESCE(friends, '[]'::jsonb)) AS elem 
                         UNION SELECT NEW.friend_id::TEXT) AS sub)
    WHERE id = NEW.user_id;

    UPDATE users
    SET friends = (SELECT jsonb_agg(elem)
                   FROM (SELECT DISTINCT jsonb_array_elements_text(COALESCE(friends, '[]'::jsonb)) AS elem 
                         UNION SELECT NEW.user_id::TEXT) AS sub)
    WHERE id = NEW.friend_id;

  ELSE
    -- Удаляем друга из массива, если статус изменился на другой
    UPDATE users
    SET friends = (SELECT jsonb_agg(elem)
                   FROM (SELECT jsonb_array_elements_text(COALESCE(friends, '[]'::jsonb)) AS elem) AS sub
                   WHERE elem::int <> NEW.friend_id)
    WHERE id = NEW.user_id;

    UPDATE users
    SET friends = (SELECT jsonb_agg(elem)
                   FROM (SELECT jsonb_array_elements_text(COALESCE(friends, '[]'::jsonb)) AS elem) AS sub
                   WHERE elem::int <> NEW.user_id)
    WHERE id = NEW.friend_id;
  END IF;

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER trigger_update_friends_list
AFTER UPDATE OF status ON friendships
FOR EACH ROW
WHEN (OLD.status IS DISTINCT FROM NEW.status)
EXECUTE FUNCTION update_friends_list();

-- privacy

CREATE TYPE VISIBILITY_ENUM as ENUM ('open', 'friendonly', 'hidden');

CREATE TABLE users_privacy (
  id INT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  profile_visibility VISIBILITY_ENUM NOT NULL DEFAULT 'open',
  friends_visibility VISIBILITY_ENUM NOT NULL DEFAULT 'open',
  can_invite VISIBILITY_ENUM NOT NULL DEFAULT 'open',
  server_visibility VISIBILITY_ENUM NOT NULL DEFAULT 'open',
  online_visibility VISIBILITY_ENUM NOT NULL DEFAULT 'open',
  hours_visibility VISIBILITY_ENUM NOT NULL DEFAULT 'open'
);
