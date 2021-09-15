CREATE TABLE IF NOT EXISTS vehicle (
    id UUID PRIMARY KEY,
    license_plate TEXT NOT NULL,
    vin TEXT NOT NULL,
    year INTEGER NOT NULL,
    make TEXT NOT NULL,
    model TEXT NOT NULL,
);