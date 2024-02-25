use image::Rgb;

pub struct Label {
    pub class_id: usize,
    pub score: f32,
}

impl Label {
    pub fn to_string(&self, label_converter: &LabelMap) -> String {
        label_converter.string_vector[self.class_id].clone()
    }

    pub fn to_color(&self, label_converter: &LabelMap) -> Rgb<u8> {
        label_converter.color_vector[self.class_id]
    }
}

pub struct LabelMap {
    string_vector: Vec<String>,
    color_vector: Vec<Rgb<u8>>,
}

impl LabelMap {
    pub fn new(string_vector: Vec<String>, color_vector: Vec<Rgb<u8>>) -> Self {
        LabelMap {
            string_vector,
            color_vector,
        }
    }

    pub fn to_id(&self, string: &str) -> usize {
        todo! {}
    }
}
