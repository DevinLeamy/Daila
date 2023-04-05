#![allow(dead_code)]
use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    activity_selector::ActivitySelectorValue,
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
    pub id: ActivityId,
    pub name: String,
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
    pub fn add_activity(&mut self, activity: Activity) {
        let date = activity.date;
        let activities = self.days.entry(date).or_insert_with(Vec::new);
        activities.push(activity);
    }

    pub fn remove_activity(&mut self, activity: Activity) {
        let activities = self.days.get_mut(&activity.date).unwrap();
        activities.retain(|a| a.activity_id != activity.activity_id);
    }

    pub fn activities_on_date(&mut self, date: CalendarDate) -> &mut Vec<Activity> {
        if !self.days.contains_key(&date) {
            self.days.insert(date, Vec::new()).unwrap();
        }

        self.days.get_mut(&date).unwrap()
    }

    pub fn activity_completed(&self, date: CalendarDate, activity_type: &ActivityType) -> bool {
        for activity in self.days.get(&date).unwrap_or(&Vec::new()) {
            if activity.activity_id == activity_type.id {
                return true;
            }
        }

        false
    }

    pub fn activities(&self) -> Vec<&Activity> {
        self.days.values().flatten().collect()
    }

    pub fn activities_with_type(&self, activity_type: &ActivityType) -> Vec<&Activity> {
        self.activities()
            .into_iter()
            .filter(|activity| activity.activity_id == activity_type.id)
            .collect()
    }
}

impl File for ActivitiesStore {
    fn path() -> PathBuf {
        PathBuf::from("/Users/Devin/Desktop/Playground/Fall2022/daila-rs/data/activities.json")
    }
}

#[derive(Clone)]
pub struct ActivityOption {
    activity_type: ActivityType,
    completed: bool,
}

impl ActivitySelectorValue for ActivityOption {
    fn name(&self) -> &str {
        self.activity_type.name.as_str()
    }

    fn completed(&self) -> bool {
        self.completed
    }
}

impl ActivityOption {
    pub fn new(activity_type: ActivityType, completed: bool) -> Self {
        Self {
            activity_type,
            completed,
        }
    }

    pub fn activity_id(&self) -> ActivityId {
        self.activity_type.id
    }
}

pub fn activity_options(
    activity_types: &ActivityTypesStore,
    activities: &ActivitiesStore,
    date: CalendarDate,
) -> Vec<ActivityOption> {
    let mut options: Vec<ActivityOption> = activity_types
        .activity_types()
        .into_iter()
        .map(|activity_type| {
            let completed = activities.activity_completed(date, activity_type);
            ActivityOption::new(activity_type.to_owned(), completed)
        })
        .collect();
    options.sort_by(|a, b| a.activity_id().0.cmp(&b.activity_id().0));

    options
}
