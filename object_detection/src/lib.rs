use carrot_common_interface::label::{Label, LabelMap};
use carrot_image_utils::geometry;
use carrot_image_utils::geometry::CarrotPoint;
use image::RgbImage;
use imageproc::{drawing, point::Point, rect::Rect};

pub struct CircleObjects {
    pub objects: Vec<Circle>,
}

pub struct Circle {
    pub label: Label,
    pub position: Point<f32>,
    pub radius: f32,
}

impl Circle {
    pub fn draw(&self, image: &mut RgbImage, label_map: &LabelMap, thickness: i32) {
        for i in 0..thickness {
            let radius = self.radius as i32 - i;
            if radius < 0 {
                break;
            }
            drawing::draw_hollow_circle_mut(
                image,
                self.position.to_i32_tuple(),
                radius,
                self.label.to_color(label_map),
            )
        }
    }

    pub fn draw_with_center_point(
        &self,
        image: &mut RgbImage,
        label_map: &LabelMap,
        thickness: i32,
    ) {
        self.draw(image, label_map, thickness);
        drawing::draw_filled_circle_mut(
            image,
            self.position.to_i32_tuple(),
            2 * thickness as i32,
            self.label.to_color(label_map),
        )
    }
}

pub struct BoxObjects {
    pub objects: Vec<Box>,
}
pub struct Box {
    pub label: Label,
    pub position: Point<f32>,
    pub width: f32,
    pub height: f32,
}

impl Box {
    pub fn to_rect(&self) -> Rect {
        geometry::to_rect(self.position, self.width as u32, self.height as u32)
    }

    pub fn draw(&self, image: &mut RgbImage, label_map: &LabelMap, thickness: i32) {
        for i in 0..thickness {
            let width = self.width as i32 - i;
            let height = self.height as i32 - i;
            if width < 0 || height < 0 {
                break;
            }
            drawing::draw_hollow_rect_mut(
                image,
                geometry::to_rect(self.position, width as u32, height as u32),
                self.label.to_color(label_map),
            );
        }
    }

    pub fn draw_with_center_point(
        &self,
        image: &mut RgbImage,
        label_map: &LabelMap,
        thickness: i32,
    ) {
        self.draw(image, label_map, thickness);
        drawing::draw_filled_circle_mut(
            image,
            self.position.to_i32_tuple(),
            2 * thickness,
            self.label.to_color(label_map),
        )
    }
}
