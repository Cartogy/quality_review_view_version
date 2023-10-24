use std::collections::HashMap;
use plotters::prelude::*;

use questionnaire::CSVWrite;

pub struct PlotData {
    data: HashMap<String,(u64,u64)>
}

impl PlotData {
    pub fn new() -> Self {
        PlotData {
            data: HashMap::new()
        }
    }

    pub fn increment_yes(&mut self, key: String) {
        if let None = self.data.get(&key) {
            self.data.insert(key.clone(), (0,0));
        }
        self.data.get_mut(&key).unwrap().0 += 1;
    }

    pub fn increment_no(&mut self, key: String) {
        if let None = self.data.get(&key) {
            self.data.insert(key.clone(), (0,0));
        }
        self.data.get_mut(&key).unwrap().1 += 1;
    }

    pub fn make_plot(&self) {
        // build the data.
        let mut data = Vec::new();
        for (k,v) in self.data.iter() {
            let percent = Self::get_valid_percentage(*v);

            data.push((k.clone(),percent));
        }


        Self::create_plot(String::from("Testing"), (1024,640), String::from("Title"), data);
    }

    fn get_valid_percentage(tuple: (u64,u64)) -> u64 {
        let total_values = (tuple.0 + tuple.1) as f32;

        let total_yes = tuple.0 as f32;

        let percent_float: f32= total_yes / total_values;

        // 
        (percent_float * 100.0) as u64
    }

    fn create_plot(file_path: String, dimensions: (u32,u32), title: String, data: Vec<(String,u64)>) -> Result<(), Box<dyn std::error::Error>>{

        let root = SVGBackend::new(&file_path, dimensions).into_drawing_area();

        root.fill(&WHITE).unwrap();
        root.margin(10,10,10,10);

        // To avoid out-of-bounds, include - 1
        let max_y = data.len() - 1;

        let mut ctx = ChartBuilder::on(&root)
            .caption(title, ("Arial", 30))
            .set_label_area_size(LabelAreaPosition::Left, 120)
            .set_label_area_size(LabelAreaPosition::Bottom,40)
            .build_cartesian_2d(0u64..100u64, (0..max_y).into_segmented())?;

        ctx.configure_mesh()
            .y_labels(max_y)
            .y_label_style(("Arial",20))
            .y_label_formatter(&|x| {
                // Extract the y value
                let x = match x {
                    SegmentValue::Exact(v) => v,
                    SegmentValue::CenterOf(v) => v,
                    SegmentValue::Last => &0,
                };

                // Map it to the section
                let op_index = usize::try_from(*x);
                let name = match op_index {
                    Ok(v) => data[v].0.clone(),
                    Err(_) => "".to_string()
                };
                format!("{}", name)
                        })
            .draw()?;

            // For some reason, I can't use *i32* for the coordinate.
        ctx.draw_series((0..).zip(data.iter()).map(|(y,x)| {
            // I think this is bottom-left of the bar chart
            let y0 = SegmentValue::Exact(y);
            // I think this is bottom-right of bar chart
            let y1 = SegmentValue::Exact(y+1);

            // I think this draws a rectangle from bottom-left to top-right.
            let mut bar = Rectangle::new([(0, y0), (x.1, y1)], RED.filled());
            bar.set_margin(5,5,0,0);
            bar
        }))?;

        Ok(())
    }

    fn to_record(&self) -> Vec<PlotDataRecord> {
        let mut records = Vec::new();

        for (k, (yes, no)) in &self.data {
            let percentage = PlotData::get_valid_percentage((*yes, *no));

            let new_record = PlotDataRecord {
                section: k.to_string(),
                ok: *yes,
                no: *no,
                percent_compliance: percentage
            };

            records.push(new_record);
        }

        records
    }
}

// For CSV writing
#[derive(Debug,serde::Serialize)]
struct PlotDataRecord {
    section: String,
    ok: u64,
    no: u64,
    #[serde(rename = "% Compliance")]
    percent_compliance: u64
}

impl CSVWrite for PlotData {
    fn write_csv(&self, file_path: String) -> Result<(), Box<dyn std::error::Error>> {
        let records = self.to_record();

        let mut wtr = csv::Writer::from_path(file_path)?;

        for record in records {
            wtr.serialize(record)?;
        }

        wtr.flush()?;
        Ok(())
            
    }
}
