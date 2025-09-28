-- Create operators table
CREATE TABLE operators (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    full_name TEXT NOT NULL,
    email TEXT,
    role TEXT NOT NULL DEFAULT 'operator',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create customers table
CREATE TABLE customers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    address TEXT,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create motorcycles table
CREATE TABLE motorcycles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    brand TEXT NOT NULL,
    model TEXT NOT NULL,
    year INTEGER NOT NULL,
    engine_cc INTEGER NOT NULL,
    vin TEXT,
    license_plate TEXT,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create test_sessions table
CREATE TABLE test_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    motorcycle_id INTEGER NOT NULL,
    customer_id INTEGER,
    operator_id INTEGER NOT NULL,
    test_type TEXT NOT NULL DEFAULT 'power_curve',
    status TEXT NOT NULL DEFAULT 'running',
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    duration_seconds INTEGER,
    max_rpm REAL,
    max_speed REAL,
    max_hp REAL,
    max_torque REAL,
    max_hp_rpm REAL,
    max_torque_rpm REAL,
    avg_oil_temp REAL,
    avg_afr REAL,
    weather_conditions TEXT,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (motorcycle_id) REFERENCES motorcycles(id),
    FOREIGN KEY (customer_id) REFERENCES customers(id),
    FOREIGN KEY (operator_id) REFERENCES operators(id)
);

-- Create test_data_points table
CREATE TABLE test_data_points (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    test_session_id INTEGER NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    time_offset_ms INTEGER NOT NULL,
    rpm REAL NOT NULL,
    speed_kmh REAL NOT NULL,
    horsepower REAL NOT NULL,
    torque_nm REAL NOT NULL,
    oil_temp_c REAL,
    air_fuel_ratio REAL,
    throttle_position REAL,
    engine_load REAL,
    FOREIGN KEY (test_session_id) REFERENCES test_sessions(id) ON DELETE CASCADE
);

-- Create calibration_settings table
CREATE TABLE calibration_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    setting_type TEXT NOT NULL,
    value REAL NOT NULL,
    unit TEXT NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX idx_test_sessions_motorcycle_id ON test_sessions(motorcycle_id);
CREATE INDEX idx_test_sessions_customer_id ON test_sessions(customer_id);
CREATE INDEX idx_test_sessions_operator_id ON test_sessions(operator_id);
CREATE INDEX idx_test_sessions_start_time ON test_sessions(start_time);
CREATE INDEX idx_test_sessions_status ON test_sessions(status);

CREATE INDEX idx_test_data_points_session_id ON test_data_points(test_session_id);
CREATE INDEX idx_test_data_points_timestamp ON test_data_points(timestamp);
CREATE INDEX idx_test_data_points_time_offset ON test_data_points(time_offset_ms);

CREATE INDEX idx_motorcycles_brand_model ON motorcycles(brand, model);
CREATE INDEX idx_customers_name ON customers(name);
CREATE INDEX idx_operators_username ON operators(username);

-- Insert default operator
INSERT INTO operators (username, full_name, role, is_active) 
VALUES ('admin', 'System Administrator', 'admin', TRUE);

-- Insert default calibration settings
INSERT INTO calibration_settings (name, setting_type, value, unit, description, is_active) VALUES
('RPM_MULTIPLIER', 'rpm_calibration', 1.0, 'factor', 'RPM sensor calibration multiplier', TRUE),
('TORQUE_OFFSET', 'torque_calibration', 0.0, 'Nm', 'Torque sensor offset calibration', TRUE),
('TORQUE_MULTIPLIER', 'torque_calibration', 1.0, 'factor', 'Torque sensor calibration multiplier', TRUE),
('TEMP_OFFSET', 'temperature_calibration', 0.0, '°C', 'Temperature sensor offset calibration', TRUE),
('SPEED_MULTIPLIER', 'speed_calibration', 1.0, 'factor', 'Speed sensor calibration multiplier', TRUE);