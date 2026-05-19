use clap::Parser;
use legal_24_nums::*;
use std::time::Instant;

/// 一个能够统计出指定范围数字内的所有能够通过加减乘除计算得到24点的组合的程序
/// (A tool that can count all combinations of numbers within a specified range that can be calculated to 24 using
/// addition, subtraction, multiplication, and division.)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// (Start number)（起始数字）
    #[arg(long, default_value_t = 1)]
    start: u32,

    /// (End number)（结束数字）
    #[arg(long, default_value_t = 10)]
    end: u32,

    /// (Generate result file)（生成结果文件）
    #[arg(short = 'f', long = "file")]
    gen_file: bool,

    /// (Generate expr in file)（生成带表达式的结果文件）
    #[arg(short = 'e', long = "expr")]
    gen_expr: bool,
}

fn main() {
    let Cli {
        start,
        end,
        gen_file,
        gen_expr,
    } = Cli::parse();

    if !gen_expr {
        let now = Instant::now();
        let results = get_all_legal_nums(start, end);
        let elapsed = now.elapsed();

        println!(
            "在{}到{}之间共找到{}个组合。(Find {} combinations between {} and {}.)",
            start,
            end,
            results.len(),
            results.len(),
            start,
            end,
        );
        println!(
            "共花费{}毫秒。(It tooks {}ms total.)",
            elapsed.as_millis(),
            elapsed.as_millis()
        );

        if gen_file {
            match gen_results_file(&results, start, end) {
                Ok(file_name) => println!(
                    "结果已保存到{}文件中。(Results has been saved into {}.)",
                    file_name, file_name
                ),
                Err(_) => {
                    println!("文件保存失败，请重试。(Generate file failed! please try again.)")
                }
            }
        }
    }

    if gen_expr {
        let now = Instant::now();
        let results = get_all_legal_nums_with_expr(start, end);
        let elapsed = now.elapsed();

        println!(
            "在{}到{}之间共找到{}个组合。(Find {} combinations between {} and {}.)",
            start,
            end,
            results.len(),
            results.len(),
            start,
            end,
        );
        println!(
            "共花费{}毫秒。(It tooks {}ms total.)",
            elapsed.as_millis(),
            elapsed.as_millis()
        );

        if gen_file {
            match gen_results_file_with_expr(&results, start, end) {
                Ok(file_name) => println!(
                    "结果已保存到{}文件中。(Results has been saved into {}.)",
                    file_name, file_name
                ),
                Err(_) => {
                    println!("文件保存失败，请重试。(Generate file failed! please try again.)")
                }
            }
        }
    }
}
