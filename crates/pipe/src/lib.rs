pub struct Pipe<In, Out> {
    mapper: Box<dyn FnOnce(In) -> Out>,
}

impl<In: 'static, Out: 'static> Pipe<In, Out> {
    pub fn new<F>(mapper: F) -> Self
    where
        F: FnOnce(In) -> Out + 'static,
    {
        Self {
            mapper: Box::new(mapper),
        }
    }

    pub fn inspect_in<F>(self, clos: F) -> Self
    where
        F: FnOnce(&In) + 'static,
    {
        Pipe::new(|t| {
            clos(&t);
            self.supply(t)
        })
    }

    pub fn inspect_out<F>(self, clos: F) -> Self
    where
        F: FnOnce(&Out) + 'static,
    {
        Pipe::new(|t| {
            let out = self.supply(t);
            clos(&out);
            out
        })
    }

    pub fn map_in<ParentIn, F>(self, mapper: F) -> Pipe<ParentIn, Out>
    where
        ParentIn: 'static,
        F: FnOnce(ParentIn) -> In + 'static,
    {
        Pipe::new(|t| self.supply(mapper(t)))
    }

    pub fn map_out<ParentOut, G>(self, mapper: G) -> Pipe<In, ParentOut>
    where
        ParentOut: 'static,
        G: FnOnce(Out) -> ParentOut + 'static,
    {
        Pipe::new(|t| mapper(self.supply(t)))
    }

    pub fn chain<NewOut, F>(self, next: Pipe<Out, NewOut>) -> Pipe<In, NewOut>
    where
        NewOut: 'static,
    {
        Pipe::new(|input| next.supply(self.supply(input)))
    }

    pub fn join<OtherIn, OtherOut>(
        self,
        other: Pipe<OtherIn, OtherOut>,
    ) -> Pipe<(In, OtherIn), (Out, OtherOut)>
    where
        OtherIn: 'static,
        OtherOut: 'static,
    {
        Pipe::new(|(t, t1)| (self.supply(t), other.supply(t1)))
    }

    pub fn split<OtherOut>(self, other: Pipe<In, OtherOut>) -> Pipe<In, (Out, OtherOut)>
    where
        In: Clone,
        OtherOut: 'static,
        OtherOut: 'static,
    {
        Pipe::new(|t: In| (self.supply(t.clone()), other.supply(t)))
    }

    pub fn supply(self, value: In) -> Out {
        (self.mapper)(value)
    }
}
