use crate::prelude::*;
use std::collections::hash_map::Entry;

pub struct System<C> {
    pub config: Arc<C>,
    components: HashMap<TypeId, Arc<dyn Component>>,
    type_ids: Option<HashSet<TypeId>>,
}

impl<C> System<C> {
    pub fn new(config: C) -> Result<Self> {
        Ok(System {
            config: Arc::new(config),
            components: HashMap::new(),
            type_ids: None,
        })
    }

    pub fn get<T>(&self) -> Result<&T>
    where
        T: Component,
    {
        let type_id = TypeId::of::<T>();
        let c = self.components.get(&type_id).unwrap();
        c.as_any().downcast_ref::<T>().context("invalid component")
    }

    pub fn set_type_ids(&mut self, type_ids: HashSet<TypeId>) {
        self.type_ids = Some(type_ids);
    }

    pub fn maybe_add<T: Component, F: FnOnce(&Self) -> Result<T>>(
        &mut self,
        type_id: &TypeId,
        f: F,
    ) -> Result<Option<Arc<T>>> {
        let should_init = self
            .type_ids
            .as_ref()
            .context("system has no typeIds")?
            .contains(type_id);

        if !should_init {
            Ok(None)
        } else {
            let component = f(self)?;
            let arc = Arc::new(component);
            let entry = self.components.entry(*type_id);
            if let Entry::Vacant(e) = entry {
                e.insert(arc.clone());
                Ok(Some(arc))
            } else {
                Err(anyhow!("typeId already included in component map"))
            }
        }
    }
}
