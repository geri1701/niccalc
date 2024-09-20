#[derive(Default, Clone)]
pub struct Model {
    shotstr: f64,
    targstr: f64,
    targvol: f64,
    aromavol: f64,
}

impl Model {
    pub fn shotstr(&self) -> f64 {
        self.shotstr
    }
    pub fn targstr(&self) -> f64 {
        self.targstr
    }
    pub fn targvol(&self) -> f64 {
        self.targvol
    }
    pub fn aromavol(&self) -> f64 {
        self.aromavol
    }
    pub fn set_shotstr(&mut self, value: f64) {
        self.shotstr = value;
    }
    pub fn set_targstr(&mut self, value: f64) {
        self.targstr = value;
    }
    pub fn set_targvol(&mut self, value: f64) {
        self.targvol = value;
    }
    pub fn set_aromavol(&mut self, value: f64) {
        self.aromavol = value;
    }
    pub fn output(&self) -> [(&str, f64); 4] {
        let shots = self.calculate_nic();
        [
            ("Nicotine Base", shots),
            ("Base", self.targvol - (shots + self.aromavol)),
            ("Flavour", self.aromavol),
            ("Total", self.targvol),
        ]
    }
    pub fn limit(&self) -> f64 {
        self.targvol - self.calculate_nic()
    }
    fn calculate_nic(&self) -> f64 {
        (self.targvol * self.targstr) / self.shotstr
    }
}
