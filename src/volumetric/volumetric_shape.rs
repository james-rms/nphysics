use na::Real;
use ncollide::shape::{Ball, Compound, ConvexHull, Cuboid, Cylinder, Shape};
use volumetric::Volumetric;
use math::{AngularInertia, Isometry, Point, Vector};

macro_rules! dispatch(
    ($p: ty, $i: ty, $sself: ident.$name: ident($($argN: ident),*)) => {
        {
            if let Some(b) = $sself.as_shape::<Ball<N>>() {
                b.$name($($argN,)*)
            }
            else if let Some(c) = $sself.as_shape::<Compound<Point<N>, Isometry<N>>>() {
                c.$name($($argN,)*)
            }
            // else if let Some(c) = $sself.as_shape::<Cone<N>>() {
            //     (c as &Volumetric<N, $p, $i>).$name($($argN,)*)
            // }
            else if let Some(c) = $sself.as_shape::<ConvexHull<Point<N>>>() {
                c.$name($($argN,)*)
            }
            else if let Some(c) = $sself.as_shape::<Cuboid<Vector<N>>>() {
                c.$name($($argN,)*)
            }
            else if let Some(c) = $sself.as_shape::<Cylinder<N>>() {
                c.$name($($argN,)*)
            }
            else {
                /*
                 * XXX: dispatch by custom type.
                 */
                panic!("The `Volumetric` is not implemented by the given shape.")
            }
        }
    }
);

impl<N: Real> Volumetric<N> for Shape<Point<N>, Isometry<N>> {
    fn area(&self) -> N {
        dispatch!(Point<N>, AngularInertia<N>, self.area())
    }

    fn volume(&self) -> N {
        dispatch!(Point<N>, AngularInertia<N>, self.volume())
    }

    fn center_of_mass(&self) -> Point<N> {
        dispatch!(Point<N>, AngularInertia<N>, self.center_of_mass())
    }

    fn unit_angular_inertia(&self) -> AngularInertia<N> {
        dispatch!(Point<N>, AngularInertia<N>, self.unit_angular_inertia())
    }

    fn mass_properties(&self, density: N) -> (N, Point<N>, AngularInertia<N>) {
        dispatch!(Point<N>, AngularInertia<N>, self.mass_properties(density))
    }
}
