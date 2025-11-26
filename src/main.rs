use crate::objective::冰雪晴芸目标函数;
use chai::config::SolverConfig;
use chai::contexts::default::默认上下文;
use chai::encoders::default::默认编码器;
use chai::interfaces::command_line::{
    从命令行参数创建, 命令, 命令行, 默认命令行参数
};
use chai::objectives::目标函数;
use chai::operators::default::默认操作;
use chai::错误;
use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::thread::spawn;

mod objective;

fn main() -> Result<(), 错误> {
    let 参数 = 默认命令行参数::parse();
    let 输入 = 从命令行参数创建(&参数);
    let 上下文 = 默认上下文::新建(输入)?;
    let _config = 上下文.配置.clone();
    match 参数.command {
        命令::Encode => {
            let 编码器 = 默认编码器::新建(&上下文)?;
            let mut 目标函数 = 冰雪晴芸目标函数::新建(&上下文, 编码器)?;
            let (指标, 分数) = 目标函数.计算(&上下文.初始映射, &None);
            println!("分数：{分数:.4}；{指标}");
            let 码表 = 上下文.生成码表(&目标函数.目标函数.编码结果);
            let 命令行 = 命令行::新建(参数, Some(".".into()));
            命令行.输出编码结果(码表);
        }
        命令::Optimize => {
            let 命令行 = 命令行::新建(参数, None);
            let 线程数 = 命令行.参数.threads.unwrap_or(1);
            let SolverConfig::SimulatedAnnealing(退火) =
                _config.optimization.unwrap().metaheuristic.unwrap();
            let mut 线程池 = vec![];
            for 线程序号 in 0..线程数 {
                let 编码器 = 默认编码器::新建(&上下文).unwrap();
                let mut 目标函数 = 冰雪晴芸目标函数::新建(&上下文, 编码器).unwrap();
                let mut 操作 = 默认操作::新建(&上下文).unwrap();
                let 优化方法 = 退火.clone();
                let 上下文 = 上下文.clone();
                let 子命令行 = 命令行.生成子命令行(线程序号);
                let 线程 = spawn(move || {
                    let 优化结果 = 优化方法.优化(
                        &上下文.初始映射,
                        &mut 目标函数,
                        &mut 操作,
                        &上下文,
                        &子命令行,
                    );
                    目标函数.计算(&优化结果.映射, &None);
                    let 码表 = 上下文.生成码表(&目标函数.目标函数.编码结果);
                    子命令行.输出编码结果(码表);
                    return 优化结果;
                });
                线程池.push(线程);
            }
            let mut 优化结果列表 = vec![];
            for (线程序号, 线程) in 线程池.into_iter().enumerate() {
                优化结果列表.push((线程序号, 线程.join().unwrap()));
            }
            优化结果列表.sort_by(|a, b| a.1.分数.partial_cmp(&b.1.分数).unwrap());
            let mut 总结文件 = File::create(命令行.输出目录.join("总结.txt"))?;
            for (线程序号, 优化结果) in 优化结果列表 {
                print!(
                    "线程 {} 分数：{:.4}；{}",
                    线程序号, 优化结果.分数, 优化结果.指标
                );
                write!(
                    总结文件,
                    "线程 {} 分数：{:.4}；{}",
                    线程序号, 优化结果.分数, 优化结果.指标
                )?;
            }
        }
    }
    Ok(())
}
