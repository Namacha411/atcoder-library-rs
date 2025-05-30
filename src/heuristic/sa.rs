use rand::Rng;
use std::fmt::Debug;

/// 状態を表すtrait
pub trait State: Clone + Debug {
    type Score: PartialOrd + Copy + Debug + Into<f64>;

    /// より良いスコアかどうか（最大化問題の場合はtrue、最小化問題の場合はfalse）
    const IS_MAXIMIZING: bool;

    /// 状態のスコアを計算
    fn score(&self) -> Self::Score;

    /// 近傍状態を生成
    fn neighbor(&self, rng: &mut impl Rng) -> Self;
}

/// 温度スケジュールを決定するtrait
pub trait TemperatureSchedule {
    /// 時刻tでの温度を返す
    fn temperature(&self, t: f64, max_t: f64) -> f64;
}

/// 線形減少スケジュール
#[derive(Debug, Clone)]
pub struct LinearSchedule {
    pub start_temp: f64,
    pub end_temp: f64,
}

impl TemperatureSchedule for LinearSchedule {
    #[inline(always)]
    fn temperature(&self, t: f64, max_t: f64) -> f64 {
        let ratio = t / max_t;
        self.start_temp * (1.0 - ratio) + self.end_temp * ratio
    }
}

/// 指数減少スケジュール
#[derive(Debug, Clone)]
pub struct ExponentialSchedule {
    pub start_temp: f64,
    pub decay_rate: f64,
}

impl TemperatureSchedule for ExponentialSchedule {
    #[inline(always)]
    fn temperature(&self, t: f64, max_t: f64) -> f64 {
        let ratio = t / max_t;
        self.start_temp * (-self.decay_rate * ratio).exp()
    }
}

/// 焼きなまし法のパラメータ
#[derive(Debug, Clone)]
pub struct AnnealingConfig {
    pub max_iterations: usize,
    pub time_limit_ms: Option<u64>,
}

/// 焼きなまし法の実行結果
#[derive(Debug, Clone)]
pub struct AnnealingResult<S: State> {
    pub best_state: S,
    pub best_score: S::Score,
    pub iterations: usize,
    pub elapsed_ms: u64,
}

/// 焼きなまし法のメイン構造体
pub struct SimulatedAnnealing<S: State, T: TemperatureSchedule> {
    config: AnnealingConfig,
    schedule: T,
    _phantom: std::marker::PhantomData<S>,
}

impl<S: State, T: TemperatureSchedule> SimulatedAnnealing<S, T> {
    pub fn new(config: AnnealingConfig, schedule: T) -> Self {
        Self {
            config,
            schedule,
            _phantom: std::marker::PhantomData,
        }
    }

    /// 焼きなまし法を実行
    pub fn run(&self, initial_state: S, rng: &mut impl Rng) -> AnnealingResult<S> {
        let start_time = std::time::Instant::now();

        let mut current_state = initial_state.clone();
        let mut current_score = current_state.score();
        let mut best_state = current_state.clone();
        let mut best_score = current_score;

        let mut iteration = 0;

        while iteration < self.config.max_iterations {
            // 時間制限チェック
            if let Some(time_limit) = self.config.time_limit_ms {
                if start_time.elapsed().as_millis() as u64 > time_limit {
                    break;
                }
            }

            let t = iteration as f64;
            let max_t = self.config.max_iterations as f64;
            let temperature = self.schedule.temperature(t, max_t);

            // 近傍状態を生成
            let neighbor_state = current_state.neighbor(rng);
            let neighbor_score = neighbor_state.score();

            // 状態遷移の判定
            let should_accept = if S::IS_MAXIMIZING {
                // 最大化問題
                if neighbor_score >= current_score {
                    true
                } else {
                    let delta =
                        Self::score_to_f64(current_score) - Self::score_to_f64(neighbor_score);
                    let prob = (-delta / temperature).exp();
                    rng.gen::<f64>() < prob
                }
            } else {
                // 最小化問題
                if neighbor_score <= current_score {
                    true
                } else {
                    let delta =
                        Self::score_to_f64(neighbor_score) - Self::score_to_f64(current_score);
                    let prob = (-delta / temperature).exp();
                    rng.gen::<f64>() < prob
                }
            };

            if should_accept {
                current_state = neighbor_state;
                current_score = neighbor_score;
            }

            // ベスト更新チェック
            let is_better = if S::IS_MAXIMIZING {
                current_score > best_score
            } else {
                current_score < best_score
            };

            if is_better {
                best_state = current_state.clone();
                best_score = current_score;
            }

            iteration += 1;
        }

        AnnealingResult {
            best_state,
            best_score,
            iterations: iteration,
            elapsed_ms: start_time.elapsed().as_millis() as u64,
        }
    }

    // スコアをf64に変換するヘルパー関数
    #[inline(always)]
    fn score_to_f64<U: Copy>(score: U) -> f64
    where
        U: Into<f64>,
    {
        score.into()
    }
}

// 便利な関数
impl<S: State> SimulatedAnnealing<S, LinearSchedule> {
    pub fn with_linear_schedule(max_iterations: usize, start_temp: f64, end_temp: f64) -> Self {
        let config = AnnealingConfig {
            max_iterations,
            time_limit_ms: None,
        };
        let schedule = LinearSchedule {
            start_temp,
            end_temp,
        };
        Self::new(config, schedule)
    }

    pub fn with_linear_schedule_and_time_limit(
        max_iterations: usize,
        time_limit_ms: u64,
        start_temp: f64,
        end_temp: f64,
    ) -> Self {
        let config = AnnealingConfig {
            max_iterations,
            time_limit_ms: Some(time_limit_ms),
        };
        let schedule = LinearSchedule {
            start_temp,
            end_temp,
        };
        Self::new(config, schedule)
    }
}

impl<S: State> SimulatedAnnealing<S, ExponentialSchedule> {
    pub fn with_exponential_schedule(
        max_iterations: usize,
        start_temp: f64,
        decay_rate: f64,
    ) -> Self {
        let config = AnnealingConfig {
            max_iterations,
            time_limit_ms: None,
        };
        let schedule = ExponentialSchedule {
            start_temp,
            decay_rate,
        };
        Self::new(config, schedule)
    }

    pub fn with_exponential_schedule_and_time_limit(
        max_iterations: usize,
        time_limit_ms: u64,
        start_temp: f64,
        decay_rate: f64,
    ) -> Self {
        let config = AnnealingConfig {
            max_iterations,
            time_limit_ms: Some(time_limit_ms),
        };
        let schedule = ExponentialSchedule {
            start_temp,
            decay_rate,
        };
        Self::new(config, schedule)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 使用例: TSP問題
    #[derive(Debug, Clone)]
    pub struct TSPState {
        pub cities: Vec<usize>,
        pub distances: Vec<Vec<f64>>,
    }

    impl State for TSPState {
        type Score = f64;
        const IS_MAXIMIZING: bool = false;

        fn score(&self) -> Self::Score {
            let mut total_distance = 0.0;
            for i in 0..self.cities.len() {
                let from = self.cities[i];
                let to = self.cities[(i + 1) % self.cities.len()];
                total_distance += self.distances[from][to];
            }
            total_distance
        }

        fn neighbor(&self, rng: &mut impl Rng) -> Self {
            let mut new_cities = self.cities.clone();

            // 2-opt操作
            let i = rng.gen_range(0..new_cities.len());
            let j = rng.gen_range(0..new_cities.len());
            if i != j {
                let (start, end) = if i < j { (i, j) } else { (j, i) };
                new_cities[start..=end].reverse();
            }

            Self {
                cities: new_cities,
                distances: self.distances.clone(),
            }
        }
    }
    #[test]
    fn test_tsp() {
        let mut rng = rand::thread_rng();

        // 4都市のテスト用距離行列
        let distances = vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ];

        let initial_state = TSPState {
            cities: vec![0, 1, 2, 3],
            distances,
        };

        let annealer = SimulatedAnnealing::with_exponential_schedule(5000, 100.0, 3.0);
        let result = annealer.run(initial_state, &mut rng);

        println!("TSP Result: {:?}", result);
        println!("Best route: {:?}", result.best_state.cities);
    }

    #[derive(Debug, Clone)]
    struct IntegerSquare {
        value: i32,
    }

    impl State for IntegerSquare {
        type Score = i32;
        const IS_MAXIMIZING: bool = false;

        fn score(&self) -> Self::Score {
            self.value * self.value
        }

        fn neighbor(&self, rng: &mut impl Rng) -> Self {
            let delta = rng.gen_range(-5..=5);
            Self {
                value: (self.value + delta).clamp(-100, 100),
            }
        }
    }
    #[test]
    fn test_with_time_limit() {
        let mut rng = rand::thread_rng();
        let initial_state = IntegerSquare { value: 50 };

        // 100ms時間制限付き
        let annealer = SimulatedAnnealing::with_linear_schedule_and_time_limit(
            1000000, // 大きな反復数
            100,     // 100ms制限
            10.0, 0.1,
        );
        let result = annealer.run(initial_state, &mut rng);

        println!("Time-limited result: {:?}", result);
        assert!(result.elapsed_ms <= 150); // 多少の誤差を許容
    }
}
