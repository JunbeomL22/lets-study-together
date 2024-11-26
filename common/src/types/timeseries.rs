use serde::{Serialize, Deserialize};
use crate::Error as CommonError; // you may want to define your error


pub type Real = f64;
pub type UnixNano = u64;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HftTimeseries {
    pub data: Vec<f64>,
    pub timestamps: Vec<UnixNano>,
}

impl HftTimeseries {
    pub fn new(data: Vec<f64>, timestamps: Vec<UnixNano>) -> Result<Self, CommonError> {
        if data.len() != timestamps.len() {
            return Err(CommonError::LengthMismatch);
        }
        Ok(HftTimeseries { data, timestamps })
    }
}

impl Default for HftTimeseries {
    fn default() -> Self {
        HftTimeseries {
            data: Vec::new(),
            timestamps: Vec::new(),
        }
    }
}

impl HftTimeseries {
    /// # Safety
    /// This function is unsafe because it does not check if the length of the data and timestamps vectors are the same.
    pub unsafe fn push_unchecked(&mut self, data: f64, timestamp: UnixNano) {
        self.data.push(data);
        self.timestamps.push(timestamp);
    }

    pub fn push(&mut self, data: f64, timestamp: UnixNano) -> Result<(), CommonError> {
        if self.data.len() != self.timestamps.len() {
            return Err(CommonError::TimestampOrderMismatch);
        }
        self.data.push(data);
        self.timestamps.push(timestamp);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl std::ops::Add for HftTimeseries {
   type Output = Self;

   fn add(self, other: Self) -> Self::Output {
       let mut result_data = Vec::new();
       let mut result_timestamps = Vec::new();
       
       let mut current_self_value = None;
       let mut current_other_value = None;
       
       let mut i = 0;
       let mut j = 0;
       
       while i < self.timestamps.len() || j < other.timestamps.len() {
           match (self.timestamps.get(i), other.timestamps.get(j)) {
               (Some(&t1), Some(&t2)) => {
                   let timestamp = std::cmp::min(t1, t2);
                   
                   if t1 == timestamp {
                       current_self_value = Some(self.data[i]);
                       i += 1;
                   }
                   if t2 == timestamp {
                       current_other_value = Some(other.data[j]);
                       j += 1;
                   }

                   if let (Some(v1), Some(v2)) = (current_self_value, current_other_value) {
                       result_timestamps.push(timestamp);
                       result_data.push(v1 + v2);
                   }
               },
               (Some(&t), None) => {
                   current_self_value = Some(self.data[i]);
                   if let Some(other_val) = current_other_value {
                       result_timestamps.push(t);
                       result_data.push(current_self_value.unwrap() + other_val);
                   }
                   i += 1;
               },
               (None, Some(&t)) => {
                   current_other_value = Some(other.data[j]);
                   if let Some(self_val) = current_self_value {
                       result_timestamps.push(t);
                       result_data.push(self_val + current_other_value.unwrap());
                   }
                   j += 1;
               },
               (None, None) => break,
           }
       }

       HftTimeseries {
           data: result_data,
           timestamps: result_timestamps,
       }
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_add_timeseries() {
       let ts1 = HftTimeseries {
           data: vec![1.0, 2.0, 3.0],
           timestamps: vec![1, 2, 3],
       };
       
       let ts2 = HftTimeseries {
           data: vec![4.0, 5.0, 6.0],
           timestamps: vec![2, 3, 4],
       };

       let result = ts1 + ts2;
       assert_eq!(result.timestamps, vec![2, 3, 4]);
       assert_eq!(result.data, vec![6.0, 8.0, 9.0]);
   }

   #[test]
   fn test_add_empty_timeseries() {
       let ts1 = HftTimeseries::default();
       let ts2 = HftTimeseries::default();

       let result = ts1 + ts2;
       assert!(result.data.is_empty());
       assert!(result.timestamps.is_empty());
   }

   #[test]
   fn test_add_non_overlapping_timeseries_with_forward_fill() {
       let ts1 = HftTimeseries {
           data: vec![1.0, 2.0],
           timestamps: vec![1, 2],
       };
       
       let ts2 = HftTimeseries {
           data: vec![3.0, 4.0],
           timestamps: vec![3, 4],
       };

       let result = ts1 + ts2;
       assert_eq!(result.timestamps, vec![3, 4]);
       assert_eq!(result.data, vec![5.0, 6.0]);
   }
}