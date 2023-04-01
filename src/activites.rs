use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    file::File,
    heatmap::{CalendarDate, HeatMapValue},
};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ActivityId(u32);

#[derive(Serialize, Deserialize, Clone)]
pub struct Activity {
    activity_id: ActivityId,
    date: CalendarDate,
}

impl Activity {
    pub fn new(activity_id: ActivityId, date: CalendarDate) -> Self {
        Self { activity_id, date }
    }
}

impl HeatMapValue for Activity {
    fn heat_map_date(&self) -> CalendarDate {
        self.date
    }

    fn heat_map_value(&self) -> f32 {
        1.0
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActivityType {
    id: ActivityId,
    name: String,
}

impl ActivityType {
    fn new(id: ActivityId, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ActivityTypesStore {
    types: HashMap<ActivityId, ActivityType>,
}

impl ActivityTypesStore {
    pub fn create_new_activity(&mut self, name: String) -> ActivityId {
        let id = self.next_unused_id();
        let activity_type = ActivityType::new(id, name);
        self.types.insert(activity_type.id, activity_type);
        id
    }

    fn next_unused_id(&self) -> ActivityId {
        let mut id = 0;
        while self.types.contains_key(&ActivityId(id)) {
            id += 1;
        }

        ActivityId(id)
    }

    pub fn activity_type(&self, id: ActivityId) -> Option<&ActivityType> {
        self.types.get(&id)
    }

    pub fn activity_types(&self) -> Vec<&ActivityType> {
        self.types.values().collect()
    }
}

impl File for ActivityTypesStore {
    fn path() -> PathBuf {
        PathBuf::from("/Users/Devin/Desktop/Playground/Fall2022/daila-rs/data/activity_types.json")
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ActivitiesStore {
    days: HashMap<CalendarDate, Vec<Activity>>,
}

impl ActivitiesStore {
    pub fn new() -> Self {
        Self {
            days: HashMap::new(),
        }
    }

    pub fn add_activity(&mut self, activity: Activity) {
        let date = activity.date;
        let activities = self.days.entry(date).or_insert_with(Vec::new);
        activities.push(activity);
    }

    pub fn activities_on_date(&mut self, date: CalendarDate) -> &mut Vec<Activity> {
        if !self.days.contains_key(&date) {
            self.days.insert(date, Vec::new()).unwrap();
        }

        self.days.get_mut(&date).unwrap()
    }

    pub fn activities(&self) -> Vec<&Activity> {
        self.days.values().flatten().collect()
    }
}

impl File for ActivitiesStore {
    fn path() -> PathBuf {
        PathBuf::from("/Users/Devin/Desktop/Playground/Fall2022/daila-rs/data/activities.json")
    }
}
