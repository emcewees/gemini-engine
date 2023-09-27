//! This module holds every struct designed to contain various ViewElements. Since every container is itself a [`ViewElement`], containers can be combined by nesting inside of each other.

use std::fmt::Debug;

use super::{utils, ColChar, Point, Vec2D, ViewElement};

/// `VisibilityToggle` is a container for a `ViewElement` with a property `visible`. When blit to the view the contained element will only appear if `visible` is `true`
pub struct VisibilityToggle<T: ViewElement> {
    pub element: T,
    pub visible: bool,
}

impl<T: ViewElement> VisibilityToggle<T> {
    pub fn new(element: T, visible: bool) -> Self {
        Self { element, visible }
    }
}

impl<T: ViewElement> ViewElement for VisibilityToggle<T> {
    fn active_pixels(&self) -> Vec<Point> {
        match self.visible {
            true => self.element.active_pixels(),
            false => vec![],
        }
    }
}

/// A `PixelContainer` only has a [`pixels`](PixelContainer::pixels) property, which gets returned directly to the View during blit
#[derive(Debug, Clone)]
pub struct PixelContainer {
    /// This is the value that gets returned by [`active_pixels()`](ViewElement::active_pixels)
    pub pixels: Vec<Point>,
}

impl PixelContainer {
    /// Create a new, empty `PixelContainer`
    pub const fn new() -> Self {
        Self { pixels: vec![] }
    }

    /// Add a single pixel to the `PixelContainer`
    pub fn push(&mut self, pixel: Point) {
        self.pixels.push(pixel);
    }

    /// Moves all the pixels into the `PixelContainer`, leaving the input empty.
    pub fn append(&mut self, pixels: &mut Vec<Point>) {
        self.pixels.append(pixels);
    }

    /// Append vector of coordinates and a single [`ColChar`] for all of them.
    pub fn append_points(&mut self, points: Vec<Vec2D>, fill_char: ColChar) {
        self.append(&mut utils::points_to_pixels(points, fill_char));
    }

    /// Plot a pixel to the PixelContainer
    pub fn plot(&mut self, pos: Vec2D, c: ColChar) {
        self.push(Point::new(pos, c))
    }

    /// Blit a [`ViewElement`] to the PixelContainer.
    pub fn blit<E: ViewElement>(&mut self, element: &E) {
        let mut active_pixels = element.active_pixels();

        self.append(&mut active_pixels);
    }
}

impl From<Vec<Point>> for PixelContainer {
    fn from(pixels: Vec<Point>) -> Self {
        Self { pixels }
    }
}

impl From<Vec<(Vec2D, ColChar)>> for PixelContainer {
    fn from(pixels: Vec<(Vec2D, ColChar)>) -> Self {
        Self {
            pixels: pixels.iter().map(|x| Point::from(*x)).collect(),
        }
    }
}

impl ViewElement for PixelContainer {
    fn active_pixels(&self) -> Vec<Point> {
        self.pixels.clone()
    }
}

/// Contains references to all added objects. Meant to be used specifically for collision calculations
pub struct CollisionContainer<'a> {
    pub elements: Vec<&'a dyn ViewElement>,
}

impl<'a> CollisionContainer<'a> {
    /// Create a new CollisionLayer
    pub const fn new() -> CollisionContainer<'a> {
        CollisionContainer { elements: vec![] }
    }

    /// Add an element to the container
    pub fn push(&mut self, element: &'a impl ViewElement) {
        self.elements.push(element)
    }

    /// Returns true if the given [`ViewElement`] is overlapping with the CollisionLayer
    pub fn overlaps_element(&self, element: &impl ViewElement) -> bool {
        self.will_overlap_element(element, Vec2D::ZERO)
    }

    /// Returns true if the element will be overlapping with the CollisionLayer when the offset is applied
    pub fn will_overlap_element(&self, element: &impl ViewElement, offset: Vec2D) -> bool {
        let collision_pixels = utils::pixels_to_points(self.active_pixels());

        for element_pixel in utils::pixels_to_points(element.active_pixels()) {
            if collision_pixels.contains(&(element_pixel + offset)) {
                return true;
            }
        }

        false
    }
}

impl<'a> From<Vec<&'a dyn ViewElement>> for CollisionContainer<'a> {
    fn from(elements: Vec<&'a dyn ViewElement>) -> Self {
        Self { elements }
    }
}

impl<'a> ViewElement for CollisionContainer<'a> {
    fn active_pixels(&self) -> Vec<Point> {
        self.elements
            .iter()
            .flat_map(|e| e.active_pixels())
            .collect()
    }
}