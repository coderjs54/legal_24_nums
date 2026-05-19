use rayon::prelude::*;
use serde_json;
use std::fs;
use std::io::Result;

// 判断数组中的数字能否通过加减乘除计算得到24点
fn is_legal_nums(nums: &Vec<f64>) -> bool {
    // 处理浮点数精度问题
    const PRECISION: f64 = 1e-7;

    let length = nums.len();
    if length == 1 {
        return (nums[0] - 24.0).abs() < PRECISION;
    }

    for i in 0..length {
        for j in 0..length {
            if i == j {
                continue;
            }

            let mut next = Vec::with_capacity(length - 1);
            for k in 0..length {
                if i != k && j != k {
                    next.push(nums[k]);
                }
            }

            let m = nums[i];
            let n = nums[j];

            // 加法和乘法满足交换律 只计算一遍即可
            if m <= n {
                // 加法
                next.push(m + n);
                if is_legal_nums(&next) {
                    return true;
                }
                next.pop();

                // 乘法
                next.push(m * n);
                if is_legal_nums(&next) {
                    return true;
                }
                next.pop();
            }

            // 减法
            next.push(m - n);
            if is_legal_nums(&next) {
                return true;
            }
            next.pop();

            // 除法
            if n.abs() > PRECISION {
                next.push(m / n);
                if is_legal_nums(&next) {
                    return true;
                }
                next.pop();
            }
        }
    }

    return false;
}

#[derive(Clone)]
struct Item {
    value: f64,
    expr: String,
}

impl Item {
    fn new(value: f64, expr: String) -> Self {
        Item { value, expr }
    }
}

// 判断数组中的数字能否通过加减乘除计算得到24点 并返回可能的计算方式
fn is_legal_nums_with_expr(nums: &Vec<f64>) -> Option<String> {
    // 处理浮点数精度问题
    const PRECISION: f64 = 1e-7;

    fn format_expr(m_expr: &str, n_expr: &str, operator: &str) -> String {
        format!("({} {} {})", m_expr, operator, n_expr)
    }

    fn solve(nums: &Vec<Item>) -> Option<String> {
        let length = nums.len();
        if length == 1 {
            let Item { value, expr } = &nums[0];
            if (value - 24.0).abs() < PRECISION {
                return Some(expr.to_string());
            } else {
                return None;
            }
        }

        for i in 0..length {
            for j in 0..length {
                if i == j {
                    continue;
                }

                let mut next = Vec::with_capacity(length - 1);
                for k in 0..length {
                    if i != k && j != k {
                        next.push(nums[k].clone());
                    }
                }

                let Item {
                    value: m,
                    expr: ref m_expr,
                } = nums[i];
                let Item {
                    value: n,
                    expr: ref n_expr,
                } = nums[j];

                // 加法和乘法满足交换律 只计算一遍即可
                if m <= n {
                    // 加法
                    next.push(Item::new(m + n, format_expr(m_expr, n_expr, "+")));
                    let result = solve(&next);
                    if result.is_some() {
                        return result;
                    }
                    next.pop();

                    // 乘法
                    next.push(Item::new(m * n, format_expr(m_expr, n_expr, "*")));
                    let result = solve(&next);
                    if result.is_some() {
                        return result;
                    }
                    next.pop();
                }

                // 减法
                next.push(Item::new(m - n, format_expr(m_expr, n_expr, "-")));
                let result = solve(&next);
                if result.is_some() {
                    return result;
                }
                next.pop();

                // 除法
                if n.abs() > PRECISION {
                    next.push(Item::new(m / n, format_expr(m_expr, n_expr, "/")));
                    let result = solve(&next);
                    if result.is_some() {
                        return result;
                    }
                    next.pop();
                }
            }
        }

        return None;
    }

    let items = nums.iter().map(|&n| Item::new(n, n.to_string())).collect();

    solve(&items)
}

// 得到指定范围数字内的所有能够通过加减乘除计算得到24点的组合够统计出指定范围数字内的所有能够通过加减乘除计算得到24点的组合
pub fn get_all_legal_nums(start: u32, end: u32) -> Vec<Vec<u32>> {
    let mut all_nums = vec![];

    for n1 in start..=end {
        for n2 in n1..=end {
            for n3 in n2..=end {
                for n4 in n3..=end {
                    all_nums.push([n1, n2, n3, n4]);
                }
            }
        }
    }

    all_nums
        .par_iter()
        .filter_map(|&nums| {
            let f64_nums: Vec<f64> = nums.iter().map(|&n| n as f64).collect();
            if is_legal_nums(&f64_nums) {
                Some(nums.to_vec())
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug, serde::Serialize)]
pub struct NumsWithExpr {
    nums: Vec<u32>,
    expr: String,
}

impl NumsWithExpr {
    pub fn new(nums: Vec<u32>, expr: String) -> Self {
        NumsWithExpr { nums, expr }
    }
}

// 得到指定范围数字内的所有能够通过加减乘除计算得到24点的组合够统计出指定范围数字内的所有能够通过加减乘除计算得到24点的组合（带表达式）
pub fn get_all_legal_nums_with_expr(start: u32, end: u32) -> Vec<NumsWithExpr> {
    let mut all_nums = vec![];

    for n1 in start..=end {
        for n2 in n1..=end {
            for n3 in n2..=end {
                for n4 in n3..=end {
                    all_nums.push([n1, n2, n3, n4]);
                }
            }
        }
    }

    all_nums
        .par_iter()
        .filter_map(|&nums| {
            let f64_nums: Vec<f64> = nums.iter().map(|&n| n as f64).collect();
            let result = is_legal_nums_with_expr(&f64_nums);
            if result.is_some() {
                Some(NumsWithExpr::new(nums.to_vec(), result.unwrap()))
            } else {
                None
            }
        })
        .collect()
}

// 将所有的结果写入到文件中
pub fn gen_results_file(results: &Vec<Vec<u32>>, start: u32, end: u32) -> Result<String> {
    // 拼接文件名
    let file_name = format!("legal_nums_from_{}_to_{}.json", start, end);

    let contents = serde_json::to_string(results)?;

    fs::write(&file_name, contents)?;

    Ok(file_name)
}

// 将带有表达式的所有的结果写入到文件中
pub fn gen_results_file_with_expr(
    results: &Vec<NumsWithExpr>,
    start: u32,
    end: u32,
) -> Result<String> {
    // 拼接文件名
    let file_name = format!("legal_nums_with_expr_from_{}_to_{}.json", start, end);

    let contents = serde_json::to_string(results)?;

    fs::write(&file_name, contents)?;

    Ok(file_name)
}
