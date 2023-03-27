use chrono::Datelike;
use tui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

pub struct CalendarDate {
    day: u32,
    month: u32,
    year: u32,
}

impl CalendarDate {
    fn today() -> Self {
        let now = chrono::Local::now();
        Self {
            day: now.day(),
            month: now.month(),
            year: now.year() as u32,
        }
    }
}

// TODO: Implement From<chrono::DateTime> for CalendarDate.
// TODO: Implement From<time::Date> for CalendarDate.

/**
 * What each tile in the heatmap represents.
 */
pub enum HeatMapTileScale {
    Day,
    Month,
    Year,
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
    fn one_year_ending_today() -> Self {
        let today = CalendarDate::today();
        let one_year_ago = CalendarDate {
            day: today.day,
            month: today.month,
            year: today.year - 1,
        };
        Self(one_year_ago, today)
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
    // The scale of the heatmap.
    tile_scale: HeatMapTileScale,
    // The range of dates displayed in the heatmap.
    date_range: HeatMapDateRange,
    // The range of heat values displayed in the heatmap.
    heat_range: HeatMapHeatRange,
    // The range of colors displayed in the heatmap.
    color_range: HeatMapColorRange,
    // The number of rows in the heatmap.
    rows: u32,
    // Values to display in the heatmap.
    values: Vec<&'a T>,
}

impl<'a, T: HeatMapValue> Default for HeatMap<'a, T> {
    fn default() -> Self {
        Self {
            tile_scale: HeatMapTileScale::Day,
            date_range: HeatMapDateRange::one_year_ending_today(),
            heat_range: HeatMapHeatRange(0.0, 255.0),
            color_range: HeatMapColorRange(Color::Black, Color::Green),
            rows: 7,
            values: vec![],
        }
    }
}

// Constructor helpers.
impl<'a, T: HeatMapValue> HeatMap<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tile_scale(mut self, tile_scale: HeatMapTileScale) -> Self {
        self.tile_scale = tile_scale;
        self
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

    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = rows;
        self
    }

    pub fn values(mut self, values: Vec<&'a T>) -> Self {
        self.values = values;
        self
    }
}

impl<'a, T: HeatMapValue> HeatMap<'a, T> {
    fn heat_at_date(&self, date: CalendarDate) -> f32 {
        todo!()
    }

    fn color_from_heat(&self, heat: f32) -> Color {
        todo!()
    }

    fn date_to_position(&self, date: CalendarDate, area: &Rect) -> (u32, u32) {
        todo!()
    }

    fn draw_date(&self, date: CalendarDate, buffer: &mut Buffer, area: &Rect) {
        todo!()
    }
}

impl<'a, T: HeatMapValue> Widget for HeatMap<'a, T> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        todo!()
    }
}
