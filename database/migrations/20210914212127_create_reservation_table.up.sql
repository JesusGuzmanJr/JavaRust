CREATE TABLE IF NOT EXISTS reservation (
    id UUID PRIMARY KEY,
    account_id UUID REFERENCES account(id) NOT NULL,
    vehicle_id UUID REFERENCES vehicle(id) NOT NULL,
    start_date_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date_time TIMESTAMP WITH TIME ZONE NOT NULL
);