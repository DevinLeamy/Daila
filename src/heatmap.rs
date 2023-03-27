use chrono::Datelike;

struct CalendarDate {
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

/**
 * What each tile in the heatmap represents.
 */
enum HeatMapTileScale {
    Day,
    Month,
    Year,
}

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
    heat_range: HeatMapHeatRange,
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
            rows: 7,
            values: vec![],
        }
    }
}
