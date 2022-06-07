// https://leetcode.cn/problems/k-closest-points-to-origin/
// 给定一个数组 points ，其中 points[i] = [xi, yi] 表示 X-Y 平面上的一个点，并且是一个整数 k ，返回离原点 (0,0) 最近的 k 个点。
// 这里，平面上两点之间的距离是 欧几里德距离（ √(x1 - x2)2 + (y1 - y2)2 ）。
// 你可以按 任何顺序 返回答案。除了点坐标的顺序之外，答案 确保 是 唯一 的

struct Solution {}

struct Point {
    distance: f32,
    point: Vec<i32>,
}

struct Points {
    points: Vec<Point>,
}

impl Points {
    fn new() -> Self {
        Points { points: Vec::new() }
    }

    fn insert(&mut self, point: Point) {
        let mut location = 0;
        for item in self.points.iter() {
            if point.distance < item.distance {
                break;
            }
            location += 1;
        }

        self.points.insert(location, point);
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut points_order = Points::new();

        println!("{:?}", points);
        for point in points {
            let r: i32 = point.iter().map(|p| p.pow(2)).sum();
            let r: f32 = (r as f32).sqrt();

            points_order.insert(Point {
                distance: r,
                point: point,
            });
        }

        for i in 0..k {
            result.push(points_order.points[i as usize].point.clone());
        }

        result
    }
}
