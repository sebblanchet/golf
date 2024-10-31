import numpy as np
import matplotlib.pyplot as plt
from scipy.integrate import solve_ivp


def fun(t, x):
    # params
    g = -9.81  # m/s2
    r = 0.04267 / 2  # m prov1
    m = 0.04593  # kg prov1
    I = 9e-6

    # drag
    C_d = 0.3
    rho = 1.12
    A = 2 * np.pi * r**2
    D = (1 / 2) * rho * C_d * r * A

    # magnus
    s = 0.000005
    M = s / m
    w_x = 100
    w_z = 0
    w_y = 110

    # state space
    xprime = np.zeros(6)
    xx = x[0]
    vx = x[1]
    yy = x[2]
    vy = x[3]
    zz = x[4]
    vz = x[5]

    # x
    xprime[0] = vx
    xprime[1] = -(D / m) * vx**2 + M * (w_z * vz - w_y * vy)

    # y
    xprime[2] = vy
    xprime[3] = g - (D / m) * vy**2 + M * (w_y * vx - w_x * vz)

    # z
    xprime[4] = vz
    xprime[5] = -(D / m) * vz**2 + M * (w_x * vy - w_z * vx)

    return xprime


# initial conditions
initial_conditions = [0, 50, 0, 15, 0, 0]
t_span = (0, 8)
t_eval = np.linspace(0, 8, 1000)

# solve the ode
sol = solve_ivp(
    fun=fun,
    t_span=t_span,
    y0=initial_conditions,
    t_eval=t_eval,
)

x = sol.y[0]

y = sol.y[2]
y[y < 0] = 0

z = sol.y[4]

# Plotting
fig = plt.figure()
ax = fig.add_subplot(111, projection="3d")

ax.plot(x, z, y, "-ob")
ax.set_title("golf ball model")
ax.set_xlim(0, 300)
ax.set_xlabel("x (m)")

ax.set_ylim(-150, 150)
ax.set_ylabel("z (m)")

ax.set_zlim(0, 20)
ax.set_zlabel("y (m)")
ax.grid(True)
plt.show()
