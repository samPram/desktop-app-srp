// @generated automatically by Diesel CLI.

diesel::table! {
    motorcycles (id) {
        id -> Integer,
        brand -> Text,
        model -> Text,
        year -> Integer,
        engine_cc -> Integer,
        vin -> Nullable<Text>,
        license_plate -> Nullable<Text>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    customers (id) {
        id -> Integer,
        name -> Text,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        address -> Nullable<Text>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    operators (id) {
        id -> Integer,
        username -> Text,
        full_name -> Text,
        email -> Nullable<Text>,
        role -> Text,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    test_sessions (id) {
        id -> Integer,
        motorcycle_id -> Integer,
        customer_id -> Nullable<Integer>,
        operator_id -> Integer,
        test_type -> Text,
        status -> Text,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        duration_seconds -> Nullable<Integer>,
        max_rpm -> Nullable<Float>,
        max_speed -> Nullable<Float>,
        max_hp -> Nullable<Float>,
        max_torque -> Nullable<Float>,
        max_hp_rpm -> Nullable<Float>,
        max_torque_rpm -> Nullable<Float>,
        avg_oil_temp -> Nullable<Float>,
        avg_afr -> Nullable<Float>,
        weather_conditions -> Nullable<Text>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    test_data_points (id) {
        id -> Integer,
        test_session_id -> Integer,
        timestamp -> Timestamp,
        time_offset_ms -> Integer,
        rpm -> Float,
        speed_kmh -> Float,
        horsepower -> Float,
        torque_nm -> Float,
        oil_temp_c -> Nullable<Float>,
        air_fuel_ratio -> Nullable<Float>,
        throttle_position -> Nullable<Float>,
        engine_load -> Nullable<Float>,
    }
}

diesel::table! {
    calibration_settings (id) {
        id -> Integer,
        name -> Text,
        setting_type -> Text,
        value -> Float,
        unit -> Text,
        description -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(test_sessions -> motorcycles (motorcycle_id));
diesel::joinable!(test_sessions -> customers (customer_id));
diesel::joinable!(test_sessions -> operators (operator_id));
diesel::joinable!(test_data_points -> test_sessions (test_session_id));

diesel::allow_tables_to_appear_in_same_query!(
    motorcycles,
    customers,
    operators,
    test_sessions,
    test_data_points,
    calibration_settings,
);