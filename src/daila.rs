use crate::activites::{ActivitiesStore, ActivityTypesStore};
use crate::file::File;

pub struct Daila;

impl Daila {
    pub fn init() -> (ActivityTypesStore, ActivitiesStore) {
        let activities_store = ActivitiesStore::load();
        let activity_types_store = ActivityTypesStore::load();

        (activity_types_store, activities_store)
    }
}
