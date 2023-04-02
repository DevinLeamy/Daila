#![allow(unused)]
use std::collections::HashMap;

use chrono::{Datelike, Days, NaiveDate};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    symbols::{
        bar::HALF,
        line::{TOP_RIGHT, VERTICAL},
    },
    text::{Span, Spans, Text},
    widgets::{List, ListItem, Paragraph, Widget},
};

pub type CalendarDate = NaiveDate;

// TODO: It would be nice to have HeatMapCell(u16, u16) and then functions:
// HeatMapCell -> (x, y)
// HeatMapCell -> CalendarDate
// CalendarDate -> HeatMapCell
//
// It would make the code easier to work with.

/**
 * What each tile in the heatmap represents.
 */
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum HeatMapTileScale {
    Day,
}

/**
 * The range of colors displayed in the heatmap.
 */
pub struct HeatMapColorRange(Color, Color);

/**
 * The range of dates displayed in the heatmap.
 */
struct HeatMapDateRange(CalendarDate, CalendarDate);

impl HeatMapDateRange {
    /**
     * One year ending today.
     */
    pub fn one_year_ending_today() -> Self {
        let today = chrono::Local::now().date_naive();
        let one_year_ago = today
            .checked_sub_signed(chrono::Duration::days(365))
            .unwrap();
        Self(one_year_ago, today)
    }

    pub fn current_year() -> Self {
        let today = chrono::Local::now().date_naive();
        let start_of_year = NaiveDate::from_ymd_opt(today.year(), 1, 1).unwrap();
        let end_of_year = NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap();
        Self(start_of_year, end_of_year)
    }
}

/**
 * The range of heat values displayed in the heatmap.
 */
struct HeatMapHeatRange(f32, f32);

pub trait HeatMapValue {
    /**
     * The date of the heatmap value.
     */
    fn heat_map_date(&self) -> CalendarDate;

    /**
     * The value of the heatmap at a given date.
     */
    fn heat_map_value(&self) -> f32;
}

pub struct HeatMap<'a, T: HeatMapValue> {
    // The range of dates displayed in the heatmap.
    date_range: HeatMapDateRange,
    // The range of heat values displayed in the heatmap.
    heat_range: HeatMapHeatRange,
    // The range of colors displayed in the heatmap.
    color_range: HeatMapColorRange,
    // The number of rows in the heatmap.
    rows: u16,
    // Values to display in the heatmap.
    values: HashMap<CalendarDate, &'a T>,
}

impl<'a, T: HeatMapValue> Default for HeatMap<'a, T> {
    fn default() -> Self {
        Self {
            date_range: HeatMapDateRange::current_year(),
            heat_range: HeatMapHeatRange(0.0, 255.0),
            color_range: HeatMapColorRange(Color::Black, Color::Green),
            rows: 7,
            values: HashMap::new(),
        }
    }
}

// Constructor helpers.
impl<'a, T: HeatMapValue> HeatMap<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn date_range(mut self, start: CalendarDate, end: CalendarDate) -> Self {
        self.date_range = HeatMapDateRange(start, end);
        self
    }

    pub fn heat_range(mut self, low_heat: f32, high_heat: f32) -> Self {
        self.heat_range = HeatMapHeatRange(low_heat, high_heat);
        self
    }

    pub fn color_range(mut self, low_heat_color: Color, high_heat_color: Color) -> Self {
        self.color_range = HeatMapColorRange(low_heat_color, high_heat_color);
        self
    }

    pub fn rows(mut self, rows: u16) -> Self {
        self.rows = rows;
        self
    }

    pub fn values(mut self, values: Vec<&'a T>) -> Self {
        self.values = values.into_iter().map(|v| (v.heat_map_date(), v)).collect();
        self
    }
}

impl<'a, T: HeatMapValue> HeatMap<'a, T> {
    fn draw_month_labels(&self, area: &Rect, buffer: &mut Buffer) {
        let mut date = self.date_range.0;
        let mut last_display_month = -1;
        while date < self.date_range.1 {
            let month = date.month() as i32;

            if last_display_month != month {
                /**
                 * Display the current month starting at the top of the
                 * heatmap starting at the leftmost column starting at that
                 * month.
                 */
                let (x, _) = self.date_to_position(date, area);
                let y = area.y;

                let month_name = date.format("%b").to_string();
                let month_text = Paragraph::new(Text::raw(&month_name));
                month_text.render(
                    Rect::new(x, y, month_name.len().try_into().unwrap(), 1),
                    buffer,
                );
                last_display_month = month;
            }

            date = date.checked_add_days(Days::new(self.rows.into())).unwrap();
        }
    }

    fn heat_at_date(&self, date: CalendarDate) -> f32 {
        match self.values.get(&date) {
            Some(value) => value.heat_map_value(),
            None => 0.0,
        }
    }

    fn color_from_heat(&self, heat: f32) -> Color {
        // TODO: LERP between the low and high colors.
        if heat == 0.0 {
            self.color_range.0
        } else {
            self.color_range.1
        }
    }

    fn date_to_position(&self, date: CalendarDate, area: &Rect) -> (u16, u16) {
        // Does not have spaces between days.
        let days_from_start = date.signed_duration_since(self.date_range.0).num_days() as u16;
        let x = area.x + days_from_start / self.rows;
        // We add one to the y coordinate to account for the month labels.
        let y = area.y + 1 + days_from_start % self.rows;
        assert!(self.position_to_date(2 * x, y, area) == date);
        (x * 2, y)
    }

    fn position_to_date(&self, x: u16, y: u16, area: &Rect) -> CalendarDate {
        let days_from_start = (x - area.x) / 2 * self.rows + (y - area.y - 1); // -1 for month labels.
        self.date_range
            .0
            .checked_add_days(Days::new(days_from_start.into()))
            .unwrap()
    }

    fn draw_date(&self, date: CalendarDate, buffer: &mut Buffer, area: &Rect) {
        let color = self.color_from_heat(self.heat_at_date(date));
        let (x, y) = self.date_to_position(date, area);
        let cell = buffer.get_mut(x, y);

        cell.set_fg(color);
        cell.set_symbol(HALF);
    }

    /**
     * Draw the border betweens months.
     */
    fn draw_date_month_border(&self, date: CalendarDate, buffer: &mut Buffer, area: &Rect) {
        let (x, y) = self.date_to_position(date, area);
        let current_month = date.month();
        let next_col_day = self.position_to_date(x + 2, y, area);

        if current_month != next_col_day.month() && next_col_day <= self.date_range.1 {
            let cell = buffer.get_mut(x + 1, y).set_fg(Color::Gray);
            if y == area.y + 1 {
                cell.set_symbol("╷");
            } else {
                cell.set_symbol(VERTICAL);
            }
        }
    }

    fn width(&self) -> u16 {
        let days = self
            .date_range
            .1
            .signed_duration_since(self.date_range.0)
            .num_days() as u16;
        days / self.rows * 2
    }

    fn height(&self) -> u16 {
        self.rows
    }
}

impl<'a, T: HeatMapValue> Widget for HeatMap<'a, T> {
    /**
     * Draw the heatmap.
     */
    fn render(self, area: Rect, buffer: &mut Buffer) {
        // Assert that there is enough space to draw the heatmap.
        assert!(area.width >= self.width());
        assert!(area.height >= self.height());

        let mut date = self.date_range.0;
        while date <= self.date_range.1 {
            self.draw_date(date, buffer, &area);
            self.draw_date_month_border(date, buffer, &area);
            date = date.checked_add_days(Days::new(1)).unwrap();
        }
        self.draw_month_labels(&area, buffer);
    }
}
