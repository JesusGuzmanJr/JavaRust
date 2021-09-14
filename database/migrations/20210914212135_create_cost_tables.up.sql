CREATE TABLE IF NOT EXISTS cost (
    id UUID PRIMARY KEY,
    vehicle_class VEHICLE_CLASS,
    vehicle_id UUID REFERENCES vehicle(id),
    usd_cents INTEGER NOT NULL,
    date TIMESTAMP WITH TIME ZONE NOT NULL
);