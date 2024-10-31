import numpy as np
import matplotlib.pyplot as plt

# Constants
m = 0.145  # mass of the ball in kg (e.g., a standard baseball)
r = 0.0375  # radius of the ball in meters
C_d = 0.47  # drag coefficient
C_m = 0.2  # Magnus coefficient (this is a rough estimate)
rho = 1.225  # air density in kg/m^3
A = np.pi * r**2  # cross-sectional area of the ball
g = np.array([0.0, 9.81, 0.0])  # gravitational acceleration vector

# Initial conditions
position = np.array([0.0, 0.0, 0.0])  # initial position (x, y, z)
velocity = np.array([10.0, 10.0, 0.0])  # initial velocity (vx, vy, vz)
angular_velocity = np.array([0.0, 0.0, 0.0])  # initial angular velocity (wx, wy, wz)

# Time parameters
dt = 0.01  # time step
total_time = 5  # total simulation time
num_steps = int(total_time / dt)

# Lists to store the trajectory for plotting
positions = []

# Simulation loop
for i in range(num_steps):
    # Calculate the speed and unit vector of velocity
    speed = np.linalg.norm(velocity)
    unit_velocity = velocity / speed

    # Drag force
    F_d = 0.5 * C_d * rho * A * speed**2 * unit_velocity

    # Magnus force
    F_m = 0.5 * C_m * rho * A * np.cross(angular_velocity, velocity)

    # Gravitational force
    F_g = m * g

    # Total force
    total_force = -F_g - F_d + F_m

    # Calculate acceleration
    acceleration = total_force / m

    # Update velocity using the trapezoidal rule
    new_velocity = velocity + acceleration * dt

    # Update position using the trapezoidal rule
    new_position = position + (velocity + new_velocity) / 2 * dt

    # Store the new position and velocity
    positions.append(new_position.copy())

    # Update the current state
    position = new_position
    velocity = new_velocity

    # ignore once it lands
    if position[1] < 0:
        print(dt * i)
        break

# Convert positions to a NumPy array for easier indexing
positions = np.array(positions)

# Plotting the trajectory
fig = plt.figure()
ax = fig.add_subplot(111, projection="3d")
ax.plot(positions[:, 0], positions[:, 2], positions[:, 1])
ax.set_xlabel("X Position (m)")
ax.set_ylabel("Z Position (m)")
ax.set_zlabel("Y Position (m)")
ax.set_title("Trajectory of a Spinning Ball (Euler Method)")
plt.show()
