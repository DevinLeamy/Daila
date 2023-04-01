use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::heatmap::{CalendarDate, HeatMapValue};

#[derive(Serialize, Deserialize, Clone)]
pub struct Activity {
    activity_type: ActivityType,
    date: CalendarDate,
}

impl Activity {
    pub fn new(activity_type: ActivityType, date: CalendarDate) -> Self {
        Self {
            activity_type,
            date,
        }
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
    name: String,
}

impl ActivityType {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Serialize, Deserialize, Clone)]
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
