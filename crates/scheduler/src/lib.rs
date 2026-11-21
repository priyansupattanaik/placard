use std::collections::VecDeque;

pub struct RotationScheduler {
    pub recent_applied_ids: VecDeque<i64>,
    pub max_recent: usize,
}

impl RotationScheduler {
    pub fn new(max_recent: usize) -> Self {
        Self {
            recent_applied_ids: VecDeque::new(),
            max_recent,
        }
    }

    /// Selects the next item avoiding the last N applied items (Section 8: N = min(10, pool size - 1))
    pub fn pick_next(&mut self, pool: &[i64]) -> Option<i64> {
        if pool.is_empty() {
            return None;
        }
        let pool_size = pool.len();
        let n = 10.min(pool_size.saturating_sub(1));

        for &id in pool {
            let is_recent = self
                .recent_applied_ids
                .iter()
                .take(n)
                .any(|&recent| recent == id);
            if !is_recent {
                self.record_applied(id);
                return Some(id);
            }
        }

        // Fallback if all are marked recent
        let chosen = pool[0];
        self.record_applied(chosen);
        Some(chosen)
    }

    pub fn record_applied(&mut self, id: i64) {
        self.recent_applied_ids.push_front(id);
        if self.recent_applied_ids.len() > self.max_recent {
            self.recent_applied_ids.pop_back();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_next_no_repeat() {
        let mut sched = RotationScheduler::new(10);
        let pool = vec![1, 2, 3];
        let p1 = sched.pick_next(&pool).unwrap();
        assert_eq!(p1, 1);
        let p2 = sched.pick_next(&pool).unwrap();
        assert_eq!(p2, 2);
        let p3 = sched.pick_next(&pool).unwrap();
        assert_eq!(p3, 3);
    }
}
