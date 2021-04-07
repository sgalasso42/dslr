#[derive(Clone)]
pub struct Data {
    pub index: usize,
    pub infos: Vec<String>,
    pub grades: Vec<Option<f32>>
}

pub struct Dataset {
    pub content: Vec<Data>,
    pub counts: Vec<usize>,
    pub means: Vec<f32>,
    pub stds: Vec<f32>,
    pub mins: Vec<f32>,
    pub quarters: Vec<f32>,
    pub halfs: Vec<f32>,
    pub three_quarters: Vec<f32>,
    pub maxs: Vec<f32>,
}

impl Dataset {
    pub fn new(content: Vec<Data>) -> Self {
        let counts: Vec<usize> = Self::get_grades_count(&content);
        let means: Vec<f32> = Self::get_grades_mean(&content, &counts);
        let stds: Vec<f32> = Self::get_grades_std(&content);
        let mins: Vec<f32> = Self::get_grades_min(&content);
        let quarters: Vec<f32> = Self::get_grades_quarter(&content);
        let halfs: Vec<f32> = Self::get_grades_half(&content);
        let three_quarters: Vec<f32> = Self::get_grades_three_quarter(&content);
        let maxs: Vec<f32> = Self::get_grades_max(&content);
        return Self {
            content: content.clone(),
            counts, means, stds, mins, quarters, halfs, three_quarters, maxs
        }
    }

    pub fn get_grades_count(content: &Vec<Data>) -> Vec<usize> {
        let mut counts: Vec<usize> = vec![];
        for feature_idx in 0..13 {
            counts.push(content.iter().filter(|data| data.grades[feature_idx].is_some()).count());
        }
        return counts;
    }

    pub fn get_grades_mean(content: &Vec<Data>, counts: &Vec<usize>) -> Vec<f32> {
        let mut means: Vec<f32> = vec![];
        for feature_idx in 0..13 {
            means.push(content.iter().filter(|data| data.grades[feature_idx].is_some()).map(|data| data.grades[feature_idx].unwrap()).sum::<f32>() / counts[feature_idx] as f32);
        }
        return means;
    }

    pub fn get_grades_std(content: &Vec<Data>) -> Vec<f32> {
        return vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    }

    pub fn get_grades_min(content: &Vec<Data>) -> Vec<f32> {
        let mut mins: Vec<f32> = vec![];
        for feature_idx in 0..13 {
            let mut min: f32 = std::f32::MAX;
            for data in content.iter().filter(|data| data.grades[feature_idx].is_some()) {
                if data.grades[feature_idx].unwrap() < min {
                    min = data.grades[feature_idx].unwrap();
                }
            }
            mins.push(min);
        }
        return mins;
    }

    pub fn get_grades_quarter(content: &Vec<Data>) -> Vec<f32> {
        return vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    }

    pub fn get_grades_half(content: &Vec<Data>) -> Vec<f32> {
        return vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    }

    pub fn get_grades_three_quarter(content: &Vec<Data>) -> Vec<f32> {
        return vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    }

    pub fn get_grades_max(content: &Vec<Data>) -> Vec<f32> {
        return vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    }
}