import numpy as np
from scipy.integrate import odeint as integrate
from matplotlib import pyplot as plot
from numpy.linalg import norm
from pydantic import BaseModel


clubs = {
    "1w": 10.5,
    "5i": 55,
    "7i": 35,
    "pw": 45,
    "lw": 60,
}


class Args(BaseModel):
    verbose: bool = False
    mass: float = 0.04593  # kg
    radius: float = 0.04267 / 2  # m
    inertia: float = 9.145e-6  # m
    clubmass: float = 0.2  # kg
    gravity: float = 9.81  # m/s2
    density: float = 1.225  # kg/m3
    viscosity: float = 1.46e-5  # -
    height: float = 0  # m
    vclub: float = 55  # m/s


def sim(args: Args):
    # Ball speed from club speed and loft angle
    def ball_speed(theta):
        theta = np.radians(theta)
        e = 0.86 - 0.0029 * args.vclub * np.cos(theta)

        bfn = (1 + e) * args.vclub * np.cos(theta) / (1 + args.mass / args.clubmass)
        bfp = (
            args.vclub
            * np.sin(theta)
            / (
                1
                + args.mass / args.clubmass
                + (args.mass * args.radius**2 / args.inertia)
            )
        )
        return np.sqrt(bfn**2 + bfp**2)

    # Spin
    def ball_spin(theta):
        theta = np.radians(theta)
        bfp = (
            args.vclub
            * np.sin(theta)
            / (
                1
                + args.mass / args.clubmass
                + (args.mass * args.radius**2 / args.inertia)
            )
        )

        s = args.mass * bfp * args.radius / args.inertia
        return s

    # Coefficient of drag from Reynolds number, based on degree four polynomial.
    def re_to_cd(re):
        # Clamp output value as it is only an approximation
        if re > 120000:
            return 0.370
        elif re < 53000:
            return 0.8

        # Array of coefficients
        coeffs = np.array(
            [
                9.46410458e-20,
                -3.80736984e-14,
                5.72048806e-09,
                -3.81337408e-04,
                9.92620188e00,
            ]
        )

        # Return value of polynomial approximation
        return np.polyval(coeffs, re)

    # Linear velocity to Reynolds number (Re = velocity * diameter / k. viscosity)
    def reynolds(velocity, radius):
        return 2 * radius * velocity / args.viscosity

    # Linear velocity to drag coefficient
    def sphere_cd(velocity, radius):
        cd = re_to_cd(reynolds(velocity, radius))
        return cd

    # Drag equation
    # F_d = 1/2 * air density * ref. area * coefficient * |velocity| * v
    def drag(density, area, cd, velocity):
        return -0.5 * density * area * cd * norm(velocity) * velocity

    # Lift equation
    # F_l = 1/2 * air density * ref. area * coefficient * |v|^2 * (what x vhat)
    def lift(density, area, cl, velocity, rvelocity):
        if cl == 0:
            return np.array([0, 0, 0])

        S = 0.5 * density * area * cl

        # Cross product of angular velocity and linear velocity, for direction of spin
        rxv = np.cross(rvelocity, velocity)
        rxv /= norm(rxv)

        # Magnitude of spin is considered in coefficient of lift
        return S * norm(velocity) ** 2 * rxv

    # Simple golfball, no drag, no lift, smooth
    class BasicGolfball:
        def __init__(self):
            # Properties
            self.mass = args.mass
            self.radius = args.radius

            # Position
            self.x = 0
            self.y = args.height
            self.z = 0

            # Velocity
            self.vx = 0
            self.vy = 0
            self.vz = 0

            # Rotational velocities
            self.rvx = 0
            self.rvy = 0
            self.rvz = 0

        # Reference area, for a sphere this is the cross-section.
        def area(self):
            return np.pi * self.radius**2

        # Set initial velocity
        def set_velocity(self, v, theta):
            self.vx = v * np.cos(np.radians(theta))
            self.vy = v * np.sin(np.radians(theta))
            self.vz = 0

        # Set spin
        def set_spin(self, spin):
            self.rvx, self.rvy, self.rvz = spin

        # Get all coordinates
        def coords(self):
            return np.array(
                [
                    self.x,
                    self.y,
                    self.z,
                    self.vx,
                    self.vy,
                    self.vz,
                    self.rvx,
                    self.rvy,
                    self.rvz,
                ]
            )

        # Set all coordinates [x, y, z, vx, vy, vz, rvx, rvy, rvz]
        def set_coords(self, coords):
            (
                self.x,
                self.y,
                self.z,
                self.vx,
                self.vy,
                self.vz,
                self.rvx,
                self.rvy,
                self.rvz,
            ) = coords

        # Returns numpy array of position coordinates
        def position(self):
            return np.array([self.x, self.y, self.z])

        # Returns numpy array of velocity at the current position
        def velocity(self):
            return np.array([self.vx, self.vy, self.vz])

        # Returns numpy array of acceleration at the current position
        def acceleration(self):
            return np.array([0, -args.gravity, 0])

        # Returns numpy array of rotational velocity (spin) at the current position
        def rvelocity(self):
            return np.array([self.rvx, self.rvy, self.rvz])

        # Returns numpy array of rotational acceleration at the current position
        def racceleration(self):
            return np.array([0, 0, 0])

        # Returns numpy array of differential eqns to be solved by odeint
        def differentials(self):
            d = np.zeros(9)

            d[0:3] = self.velocity()
            d[3:6] = self.acceleration()

            d[6:9] = self.racceleration()

            return d

        # (Internal) Updates coordinates and returns list of equations to solve (for odeint)
        def __eqns(self, t, coords):
            self.set_coords(coords)

            if args.verbose:
                print(
                    t,
                    self.velocity(),
                    self.rvelocity(),
                    self.acceleration(),
                    self.racceleration(),
                )

            return self.differentials()

        # Solve for trajectory over given interval
        def solve(self, t0, t1, dt=0.01):
            interval = np.linspace(t0, t1, int((t1 - t0) / dt))
            res = integrate(self.__eqns, self.coords(), interval, tfirst=True)
            out = np.array([e for e in res if e[1] >= 0])
            return out

    # Simple golf ball but with drag
    class DragGolfball(BasicGolfball):
        def __init__(self):
            BasicGolfball.__init__(self)

        # Coefficient of drag from velocity & radius
        def cd(self):
            return sphere_cd(norm(self.velocity()), self.radius)

        def acceleration(self):
            fd = drag(args.density, self.area(), self.cd(), self.velocity())
            return BasicGolfball.acceleration(self) + fd / self.mass

    # Golfball with lift and drag
    class LiftGolfball(DragGolfball):
        def __init__(self):
            DragGolfball.__init__(self)

        # Returns spin factor
        def spinf(self):
            v = norm(self.velocity())
            w = self.radius * norm(self.rvelocity())
            return w / v

        # Returns coefficient of lift based on spin factor
        def cl(self):
            s = self.spinf()
            return -3.25 * s**2 + 1.99 * s

        def acceleration(self):
            fl = lift(
                args.density, self.area(), self.cl(), self.velocity(), self.rvelocity()
            )
            return DragGolfball.acceleration(self) + fl / self.mass

        # Spin decreases by about 1% every second
        def racceleration(self):
            return -0.01 * self.rvelocity()

    # Initial conditions
    for theta in np.arange(10, 45, 5):
        ball = LiftGolfball()
        ball.set_velocity(ball_speed(theta), theta)
        ball.set_spin([ball_spin(0), ball_spin(2), ball_spin(theta)])

        res = ball.solve(0, 10)
        x, y, z, vx, vy, vz, rx, ry, rz = res.T

        plot.figure(1)
        plot.plot(x, y, label=f"Loft angle: {theta}")
        plot.figure(2)
        plot.plot(x, z, label=f"Loft angle: {theta}")

    plot.legend()
    plot.grid(True)
    plot.xlabel("distance (m)")
    plot.ylabel("height (m)")
    plot.title(
        "ballistic trajectory for air density "
        + format(args.density, ".3f")
        + " kg/m^3"
    )
    plot.show()


if __name__ == "__main__":
    # params
    args = Args()
    sim(args)
