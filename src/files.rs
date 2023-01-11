use std::{path, fs};

use rfd::FileDialog;

use crate::map_data;

#[derive(Clone, Debug)]
pub(crate) struct HypAbFile {
    path: Option<path::PathBuf>,
    pub map_data: map_data::RootData,
}

impl HypAbFile {
    pub(crate) fn new() -> Self {
        Self {
            path: None,
            map_data: map_data::RootData::new(),
        }
    }

    pub(crate) fn title(&self) -> String {
        if let Some(p) = &self.path {
            format!("hyperabstract -- {}: {}", &self.map_data.name, p.file_name().unwrap().to_str().unwrap())
        }
        else {
            format!("hyperabstract -- {}", &self.map_data.name)
        }
    }

    pub(crate) fn update_title(&mut self, new_title: String) {
        self.map_data.name = new_title;
    }

    pub(crate) fn query_file() -> Option<Self> {
        let res = FileDialog::new()
            .add_filter("map", &["hypab"])
            .pick_file();

        if let Some(p) = res {
            let reader: String = fs::read_to_string(&p).unwrap_or_else(|_| String::new());
            let derserialized = serde_json::from_str(&reader).unwrap_or_else(|_| map_data::RootData::new());
            Some(Self {
                path: Some(p),
                map_data: derserialized,
            })
        }
        else {
            None
        }
    }

    pub(crate) fn save_file(&mut self) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self.map_data)?;
        if let Some(p) = &self.path {
            fs::write(p, serialized)?;
        }
        else {
            self.save_as_intermediate(serialized)?;
        }
        Ok(())
    }

    pub(crate) fn save_file_as(&mut self) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self.map_data)?;
        self.save_as_intermediate(serialized)?;
        Ok(())
    }

    fn save_as_intermediate(&mut self, content: String) -> std::io::Result<()> {
        let res = FileDialog::new()
            .add_filter("map", &["hypab"])
            .set_file_name(&self.map_data.name)
            .save_file();

        if let Some(p) = res {
            fs::write(&p, content)?;
            self.path = Some(p);
        }

        Ok(())
    }
}