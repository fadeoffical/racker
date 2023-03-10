pub(crate) fn default() -> Heads {
    Heads::new()
}

#[derive(Clone)]
pub(crate) struct Head {
    pub(crate) name: String,
}

pub(crate) struct Heads {
    pub(crate) heads: Vec<Head>,
}

impl Heads {
    pub(crate) fn new() -> Self {
        Self { heads: vec![] }
    }

    pub(crate) fn register_head(&mut self, head: Head) -> Result<(), ()> {
        if self.has_head(head.name.clone()) {
            return Err(());
        }

        self.heads.push(head);
        Ok(())
    }

    pub(crate) fn unregister_head(&mut self, name: String) -> Result<(), ()> {
        if !self.has_head(name.clone()) {
            return Err(());
        }

        self.heads.retain(|head| head.name != name);
        Ok(())
    }

    pub(crate) fn has_head(&self, name: String) -> bool {
        self.heads.iter().any(|head| head.name == name)
    }

    pub(crate) fn get_all(&self) -> &Vec<Head> {
        &self.heads
    }
}
