use std::cmp::max;

#[derive(Debug, Clone)]
pub enum Ty {
    Numeric(Numeric),
    Array(Array),
    Struct(Struct),
    Union(Union),
}

impl Ty {
    pub fn size(&self) -> Option<usize> {
        match self {
            Ty::Numeric(n) => Some(n.size()),
            Ty::Array(a) => a.size(),
            Ty::Struct(s) => s.size(),
            Ty::Union(u) => u.size(),
        }
    }

    pub fn alignment(&self) -> Option<usize> {
        match self {
            Ty::Numeric(n) => Some(n.alignment()),
            Ty::Array(a) => a.alignment(),
            Ty::Struct(s) => s.alignment(),
            Ty::Union(u) => u.alignment(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Numeric {
    Integer32,
    Unsigned32,
    Float32,
}

impl Numeric {
    fn size(&self) -> usize {
        4
    }

    fn alignment(&self) -> usize {
        4
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    ty: Box<Ty>,
    length: usize,
}

impl Array {
    fn size(&self) -> Option<usize> {
        self.ty.size().map(|s| s * self.length)
    }

    fn alignment(&self) -> Option<usize> {
        self.ty.alignment()
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    name: String,
    fields: Vec<Ty>,
}

impl Struct {
    fn size(&self) -> Option<usize> {
        // Returns the amount of padding needed to get offset to alignment
        fn get_padding(offset: usize, alignment: usize) -> usize {
            let remainder = offset % alignment;
            alignment - remainder
        }

        let mut size = 0;

        for field in &self.fields {
            // Add padding so the field is aligned
            size += get_padding(size, field.alignment()?);
            // Add the field size
            size += field.size()?;
        }

        // Add padding so the whole struct is aligned
        size += get_padding(size, self.alignment()?);

        Some(size)
    }

    fn alignment(&self) -> Option<usize> {
        self.fields
            .iter()
            .map(|f| f.alignment())
            .try_fold(1, |acc, alignment| Some(max(acc, alignment?)))
    }
}

#[derive(Debug, Clone)]
pub struct Union {
    name: String,
    variants: Vec<Struct>,
    tagged: bool,
}

impl Union {
    fn size(&self) -> Option<usize> {
        self.variants
            .iter()
            .filter_map(|s| s.size())
            .max()
            .map(|size| size + if self.tagged { 1 } else { 0 })
    }

    fn alignment(&self) -> Option<usize> {
        self.variants.iter().filter_map(|s| s.alignment()).max()
    }
}
