-- init-db.sql
CREATE TABLE IF NOT EXISTS botscheduler (
    id BIGSERIAL PRIMARY KEY,
    guild_id BIGINT UNIQUE,
    channel_id BIGINT,
    timetable_message_id BIGINT,
    react_message_id BIGINT
);

CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT,
    guild_id BIGINT REFERENCES botscheduler(guild_id)
);

CREATE TABLE IF NOT EXISTS user_time_slots (
    id BIGSERIAL PRIMARY KEY,
    user_record_id BIGINT REFERENCES users(id),
    day VARCHAR(255),
    available BOOLEAN
);
