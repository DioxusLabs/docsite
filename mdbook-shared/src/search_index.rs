use std::{fmt::Debug, hash::Hash, ops::Deref};

use instant_distance::{Builder, HnswMap, Search};
use serde::{Deserialize, Serialize};

use crate::MdBook;

#[derive( Deserialize, Serialize)]
pub struct SearchIndex<T> {
    model: HnswMap<Embedding, T>,
}

impl<T> Debug for SearchIndex<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchIndex")
            .finish()
    }
}

impl<T: Clone + PartialEq> SearchIndex<T> {
    pub fn from_book(book: &MdBook<T>) -> Self where T: Hash+Eq{
        let mut points = Vec::new();
        let mut values = Vec::new();

        for (url, page) in &book.pages {
            points.push(page.embedding.clone());
            values.push(url.clone());
        }

        Self::new(points, values)
    }

    pub fn new(points: Vec<Embedding>, values: Vec<T>) -> Self {
        let points = points.into_iter().map(|e| e).collect();
        let model = Builder::default().build(points, values);

        SearchIndex { model }
    }

    pub fn get_closest(&self, embedding: Embedding, n: usize) -> Vec<T> {
        let mut search = Search::default();
        self.model
            .search(&embedding, &mut search)
            .take(n)
            .map(|result| result.value.clone())
            .collect()
    }
}

impl instant_distance::Point for Embedding {
    fn distance(&self, other: &Self) -> f32 {
        1. - cosine_similarity(&self.vector, &other.vector)
    }
}

fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    let dot_product = dot(v1, v2);
    let magnitude1 = magnitude(v1);
    let magnitude2 = magnitude(v2);

    dot_product / (magnitude1 * magnitude2)
}

fn dot(v1: &[f32], v2: &[f32]) -> f32 {
    v1.iter().zip(v2.iter()).map(|(&x, &y)| x * y).sum()
}

fn magnitude(v: &[f32]) -> f32 {
    v.iter().map(|&x| x * x).sum::<f32>().sqrt()
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
/// A document embedding for a page.
pub struct Embedding {
    vector: Vec<f32>,
}


impl Embedding{
    pub fn new(vector: Vec<f32>) -> Self{
        Self{
            vector
        }
    }
}

impl Deref for Embedding {
    type Target = Vec<f32>;

    fn deref(&self) -> &Self::Target {
        &self.vector
    }
}