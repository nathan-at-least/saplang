use crate::{Element, IntoIter, Unbundled};
use sappho_identmap::{IdentMap, TryIntoIdentMap};
use sappho_unparse::{Stream, Unparse};

#[derive(Clone, Debug, PartialEq, derive_more::Constructor)]
pub struct Object<F, Q, P, A> {
    f: Option<F>,
    q: Option<Q>,
    p: Option<P>,
    a: IdentMap<A>,
}

impl<F, Q, P, A> Default for Object<F, Q, P, A> {
    fn default() -> Self {
        Object::new(None, None, None, IdentMap::default())
    }
}

impl<F, Q, P, A> Object<F, Q, P, A> {
    pub fn new_func(func: F) -> Self {
        Self::new(Some(func), None, None, IdentMap::default())
    }

    pub fn new_query(query: Q) -> Self {
        Self::new(None, Some(query), None, IdentMap::default())
    }

    pub fn new_proc(proc: P) -> Self {
        Self::new(None, None, Some(proc), IdentMap::default())
    }

    pub fn new_attrs<T>(attrs: T) -> Self
    where
        T: Into<IdentMap<A>>,
    {
        Self::new(None, None, None, attrs.into())
    }

    pub fn func(&self) -> Option<&F> {
        self.f.as_ref()
    }

    pub fn query(&self) -> Option<&Q> {
        self.q.as_ref()
    }

    pub fn proc(&self) -> Option<&P> {
        self.p.as_ref()
    }

    pub fn attrs(&self) -> &IdentMap<A> {
        &self.a
    }

    pub fn unbundle(self) -> Unbundled<F, Q, P, A> {
        use Unbundled::*;

        match self {
            Object {
                f: None,
                q: None,
                p: None,
                a,
            } => Attrs(a),

            Object {
                f: Some(f),
                q: None,
                p: None,
                a,
            } if a.is_empty() => Func(f),

            Object {
                f: None,
                q: Some(q),
                p: None,
                a,
            } if a.is_empty() => Query(q),

            Object {
                f: None,
                q: None,
                p: Some(p),
                a,
            } if a.is_empty() => Proc(p),

            bundle => Bundled(bundle),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.f.is_none() && self.q.is_none() && self.p.is_none() && self.a.is_empty()
    }

    pub fn transform<TF, FR, TQ, QR, TP, PR, TA, AR>(
        self,
        tfunc: TF,
        tquery: TQ,
        tproc: TP,
        tattr: TA,
    ) -> Object<FR, QR, PR, AR>
    where
        TF: FnOnce(F) -> FR,
        TQ: FnOnce(Q) -> QR,
        TP: FnOnce(P) -> PR,
        TA: Fn(A) -> AR,
    {
        Object {
            f: self.f.map(tfunc),
            q: self.q.map(tquery),
            p: self.p.map(tproc),
            a: self.a.into_map_values(tattr),
        }
    }

    pub fn into_try_map_values<TA, DA, E>(self, tattr: TA) -> Result<Object<F, Q, P, DA>, E>
    where
        TA: Fn(A) -> Result<DA, E>,
    {
        let mut dsta = IdentMap::default();
        for (aname, x) in self.a {
            let dx = tattr(x)?;
            dsta.define(aname, dx).unwrap();
        }
        Ok(Object::new(self.f, self.q, self.p, dsta))
    }

    pub fn as_refs(&self) -> Object<&F, &Q, &P, &A> {
        Object {
            f: self.f.as_ref(),
            q: self.q.as_ref(),
            p: self.p.as_ref(),
            a: self.a.iter().map(|(k, v)| (k.clone(), v)).collect(),
        }
    }
}

impl<F, Q, P, A> IntoIterator for Object<F, Q, P, A> {
    type Item = Element<F, Q, P, A>;
    type IntoIter = IntoIter<F, Q, P, A>;

    fn into_iter(self) -> Self::IntoIter {
        let Object { f, q, p, a } = self;
        IntoIter {
            f,
            q,
            p,
            a: a.into_iter(),
        }
    }
}

impl<F, Q, P, A> FromIterator<Element<F, Q, P, A>> for Result<Object<F, Q, P, A>, String> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Element<F, Q, P, A>>,
    {
        use Element::*;

        let mut obj = Object::default();
        for elem in iter {
            match elem {
                Func(f) => {
                    if obj.f.replace(f).is_some() {
                        return Err("multiple funcs disallowed in object creation".to_string());
                    }
                }
                Query(q) => {
                    if obj.q.replace(q).is_some() {
                        return Err("multiple queries disallowed in object creation".to_string());
                    }
                }
                Proc(p) => {
                    if obj.p.replace(p).is_some() {
                        return Err("multiple procs disallowed in object creation".to_string());
                    }
                }
                Attr(k, v) => obj
                    .a
                    .define(k.clone(), v)
                    .map_err(|_| format!("duplicate attribute {:?}", k))?,
            }
        }
        Ok(obj)
    }
}

impl<F, Q, P, A> TryIntoIdentMap<A> for Object<F, Q, P, A> {
    fn try_into_identmap(&self) -> Option<&IdentMap<A>> {
        if self.f.is_none() && self.q.is_none() && self.p.is_none() {
            Some(self.attrs())
        } else {
            None
        }
    }
}

impl<F, Q, P, A> Unparse for Object<F, Q, P, A>
where
    F: Unparse,
    Q: Unparse,
    P: Unparse,
    A: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::Brackets::Squiggle;
        use sappho_unparse::Break::OptSpace;

        if self.is_empty() {
            s.write("{}");
        } else {
            s.bracketed(Squiggle, |subs| {
                for elem in self.as_refs().into_iter() {
                    subs.write(&OptSpace);
                    subs.write(&elem);
                    subs.write(",");
                }
            });
        }
    }
}
