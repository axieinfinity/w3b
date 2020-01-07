use std::{cmp::Ordering, fs::File, io, io::Write, path::PathBuf};

const BASE_PATH: &'static str = env!("CARGO_MANIFEST_DIR");
const PATH: &'static str = "src/numeric/numeric.rs";

fn main() -> io::Result<()> {
    let path = PathBuf::from(BASE_PATH).join(PATH);
    let mut file = File::create(path).unwrap();

    writeln!(file, "use crate::__impl_num;")?;

    for size in (8..=256).step_by(8) {
        writeln!(file)?;
        Numeric(Kind::Int, size).r#impl(&mut file)?;
    }

    for size in (8..=256).step_by(8) {
        writeln!(file)?;
        Numeric(Kind::Uint, size).r#impl(&mut file)?;
    }

    writeln!(file)?;
    writeln!(file, "#[cfg(has_i128)]")?;
    writeln!(file, "const _I128_IMPLS: () = {{")?;

    for size in (8..=256).step_by(8) {
        Numeric(Kind::Int, size).impl_128(&mut file)?;
    }

    writeln!(file)?;

    for size in (8..=256).step_by(8) {
        Numeric(Kind::Uint, size).impl_128(&mut file)?;
    }

    writeln!(file, "}};")
}

const PRIMITIVES: [Numeric; 8] = [
    Numeric(Kind::Int, 8),
    Numeric(Kind::Int, 16),
    Numeric(Kind::Int, 32),
    Numeric(Kind::Int, 64),
    Numeric(Kind::Uint, 8),
    Numeric(Kind::Uint, 16),
    Numeric(Kind::Uint, 32),
    Numeric(Kind::Uint, 64),
];

const PRIMITIVES_128: [Numeric; 2] = [Numeric(Kind::Int, 128), Numeric(Kind::Uint, 128)];
const ORDERINGS: [Ordering; 3] = [Ordering::Greater, Ordering::Equal, Ordering::Less];

#[derive(PartialEq)]
enum Kind {
    Int,
    Uint,
}

impl Kind {
    pub fn ty(&self) -> &str {
        match self {
            Kind::Int => "Int",
            Kind::Uint => "Uint",
        }
    }

    pub fn primitive(&self) -> &str {
        match self {
            Kind::Int => "i",
            Kind::Uint => "u",
        }
    }

    pub fn directive(&self) -> &str {
        match self {
            Kind::Int => "@int",
            Kind::Uint => "@uint",
        }
    }
}

struct Numeric(pub Kind, pub u16);

impl Numeric {
    pub fn fits_in(&self, other: &Self) -> Ordering {
        if self.1 != other.1 {
            return self.1.cmp(&other.1);
        }

        if self.0 != other.0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }

    pub fn query_types<'a>(
        &self,
        ord: Ordering,
        types: impl Iterator<Item = &'a Numeric>,
    ) -> String {
        types
            .filter(|primitive| self.fits_in(*primitive) == ord)
            .map(|primitive| format!("{}{}", primitive.0.primitive(), primitive.1))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn r#impl(&self, writer: &mut impl io::Write) -> io::Result<()> {
        writeln!(writer, "__impl_num! {{")?;
        writeln!(writer, "    {}{};", self.0.ty(), self.1)?;
        writeln!(writer, "    {}, size = {};", self.0.directive(), self.1 / 8)?;

        for ord in ORDERINGS.iter() {
            let primitives = self.query_types(*ord, PRIMITIVES.iter());

            if !primitives.is_empty() {
                match ord {
                    Ordering::Greater => writeln!(writer, "    @gt {};", primitives)?,
                    Ordering::Equal => writeln!(writer, "    @eq {};", primitives)?,
                    Ordering::Less => writeln!(writer, "    @lt {};", primitives)?,
                }
            }
        }

        writeln!(writer, "}}")
    }

    pub fn impl_128(&self, writer: &mut impl io::Write) -> io::Result<()> {
        write!(writer, "    __impl_num!({}{}", self.0.ty(), self.1)?;

        for ord in ORDERINGS.iter() {
            let primitives = self.query_types(*ord, PRIMITIVES_128.iter());

            if !primitives.is_empty() {
                match ord {
                    Ordering::Greater => write!(writer, "; @gt {}", primitives)?,
                    Ordering::Equal => write!(writer, "; @eq {}", primitives)?,
                    Ordering::Less => write!(writer, "; @lt {}", primitives)?,
                }
            }
        }

        writeln!(writer, ");")
    }
}
