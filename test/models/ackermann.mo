model Ackermann "Ackermann rover"
    parameter Real
        wheel_seperation = 0.1,
        wheel_base = 0.2,
        wheel_radius = 0.3,
        wheel_width = 0.4,
        wheel_mass = 0.5,
        wheel_max_turn_angle = 0.6,
        fuselage_mass = 0.7,
        fuselage_width = 0.8,
        fuselage_height = 0.9,
        fuselage_length = 0.10,
        wheel_max_rotational_rate = 1,
        wheel_inertia_ixx = (wheel_mass / 2) * (wheel_radius ^ 2);
    output Real x, y, theta;
    input Real u, omega;
equation
    der(x) = u*cos(theta);
    der(y) = u*sin(theta);
    der(theta) = omega;
end Ackermann;
