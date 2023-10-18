-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TYPE propertytypeenum AS ENUM ('STRING', 'FLOAT', 'BOOL');

CREATE TYPE shapetypeenum AS ENUM ('CIRCLE', 'ELLIPSE', 'GEOJSON');

CREATE TABLE entities (
        uid UUID NOT NULL,
        source TEXT NOT NULL,
        source_id TEXT NOT NULL,
        reference_uid UUID,
        ts_created TIMESTAMP WITHOUT TIME ZONE,
        ts_updated TIMESTAMP WITHOUT TIME ZONE,
        ts_begin TIMESTAMP WITHOUT TIME ZONE,
        ts_end TIMESTAMP WITHOUT TIME ZONE,
        stale_secs INTEGER,
        delete_secs INTEGER,
        stale BOOLEAN,
        latitude_degrees FLOAT NOT NULL,
        longitude_degrees FLOAT NOT NULL,
        altitude_msl_meters FLOAT NOT NULL,
        height_agl_meters FLOAT NOT NULL,
        height_agl_min_meters FLOAT NOT NULL,
        heading_degrees FLOAT NOT NULL,
        speed_knots FLOAT NOT NULL,
        pitch_degrees FLOAT NOT NULL,
        roll_degrees FLOAT NOT NULL,
        shape_type shapetypeenum,
        shape_latitude_degrees FLOAT,
        shape_longitude_degrees FLOAT,
        shape_radius_meters FLOAT,
        shape_minor_radius_meters FLOAT,
        shape_orientation_degrees FLOAT,
        shape_geojson TEXT,
        shape geography(Geometry,4326) NOT NULL,
        entity_type TEXT,
        owner_key TEXT,
        field_key TEXT,
        locked_property_key TEXT,
        unlocked_property_key TEXT,
        PRIMARY KEY (uid),
        UNIQUE (uid),
        FOREIGN KEY(reference_uid) REFERENCES entities (uid)
);

-- CREATE INDEX idx_entities_shape ON entities USING gist (shape)

CREATE TABLE properties (
        uid UUID NOT NULL,
        name TEXT NOT NULL,
        prop_type propertytypeenum NOT NULL,
        str_value TEXT,
        float_value FLOAT,
        bool_value BOOLEAN,
        locked BOOLEAN NOT NULL,
        PRIMARY KEY (uid, name),
        FOREIGN KEY(uid) REFERENCES entities (uid) ON DELETE CASCADE
);




