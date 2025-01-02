use std::collections::HashMap;


use crate::{errors::ZyppResult, select_resolvable, unselect_resolvable, ResolvableKind};

/// Captures list of required resolvables
pub struct Requirements {
    /// Type of resolvable
    /// If requirement needs different kinds, then define
    ///  more areas like bootloader_patterns, bootloader_packages
    kind: ResolvableKind,
    /// List of resolvables
    resolvables: Vec<String>,
    /// if not satisfied requirement is fatal or not
    optional: bool,
}

impl Requirements {
    pub fn select(&self) -> ZyppResult<()> {
        for res in &self.resolvables {
            select_resolvable(&res, self.kind, crate::ResolvableSelected::Installation)?;
            // TODO: do not fail for missing optional package
        }
        Ok(())
    }

    pub fn unselect(&self) -> ZyppResult<()> {
        for res in &self.resolvables {
            unselect_resolvable(&res, self.kind, crate::ResolvableSelected::Installation)?;
            // TODO: do not fail for missing optional package
        }
        Ok(())
    }
}

/// Keeps requirements for different areas
pub struct Areas {
    map: HashMap<String, Requirements>,
}

impl Areas {
    /// Sets for given id requirements. It will try to satisfy requirements and also if given
    /// id contain old requirements, unselects them.
    pub fn set_resolvables(&mut self, id: &str, requirements: Requirements) -> ZyppResult<()> {
        let key = id.to_string();
        if let Some(old) = self.map.get(&key) {
            old.unselect()?;
        }
        requirements.select()?;
        self.map.insert(key, requirements);
        Ok(())
    }
}