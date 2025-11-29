use chai::{
    contexts::default::默认上下文,
    encoders::default::默认编码器,
    objectives::{default::默认目标函数, metric::默认指标, 目标函数},
    元素映射, 编码信息, 错误,
};
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

type Code = [u8; 4];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct 冰雪晴芸指标 {
    指标: 默认指标,
    第一冲突: usize,
    潜在冲突: usize,
}

impl Display for 冰雪晴芸指标 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "第一冲突：{}；潜在冲突：{}；{}", self.第一冲突, self.潜在冲突, self.指标)
    }
}

pub struct 冰雪晴芸目标函数 {
    pub 目标函数: 默认目标函数<默认编码器>,
    pub 进制: u64,
}

impl 目标函数 for 冰雪晴芸目标函数 {
    type 目标值 = 冰雪晴芸指标;
    type 解类型 = 元素映射;
    fn 计算(
        &mut self, 映射: &元素映射, 变化: &Option<Vec<usize>>
    ) -> (Self::目标值, f64) {
        let (指标, 分数) = self.目标函数.计算(映射, 变化);
        let (第一冲突, 潜在冲突) = self.计算冲突数(&self.目标函数.编码结果);
        let 分数 = 分数 + 第一冲突 as f64 * 0.01 + 潜在冲突 as f64 * 0.1;
        (冰雪晴芸指标 { 指标, 第一冲突, 潜在冲突 }, 分数)
    }
}

impl 冰雪晴芸目标函数 {
    pub fn 新建(上下文: &默认上下文, 编码器: 默认编码器) -> Result<Self, 错误> {
        Ok(Self {
            目标函数: 默认目标函数::新建(上下文, 编码器)?,
            进制: 上下文.棱镜.进制,
        })
    }

    fn 计算冲突数(&self, 编码结果: &[编码信息]) -> (usize, usize) {
        let mut 编码列表 = vec![];
        // C
        let mut 编码集合 = FxHashSet::default();
        for 编码信息 in 编码结果 {
            let mut arr = [0u8; 4];
            let mut 编码 = 编码信息.全码.原始编码;
            let mut index = 0;
            while 编码 != 0 {
                arr[index] = (编码 % self.进制) as u8;
                编码 /= self.进制;
                index += 1;
            }
            编码列表.push(arr);
            编码集合.insert(arr);
        }
        编码列表.sort_by(|s1, s2| s1.cmp(s2));

        // 计算 S1
        let mut 第一后缀集合 = FxHashSet::default();
        let mut 第一冲突 = 0;
        for k in 0..(编码列表.len() - 1) {
            let s1 = 编码列表[k];
            let s2 = 编码列表[k + 1];
            if s1 == s2 {
                continue;
            }
            if let Some(diff) = difference(&s1, &s2) {
                if 编码集合.contains(&diff) {
                    第一冲突 += 1;
                } else {
                    第一后缀集合.insert(diff);
                }
            }
        }

        // 计算 U
        let mut 新编码列表 = vec![];
        for 编码 in 编码集合.iter() {
            新编码列表.push((*编码, 0));
        }
        for 编码 in 第一后缀集合.iter() {
            新编码列表.push((*编码, 1));
        }
        新编码列表.sort_by(|s1, s2| s1.cmp(s2));

        // 计算 S2
        let mut 潜在冲突 = 0;
        for k in 0..(新编码列表.len() - 1) {
            let (s1, t1) = 新编码列表[k];
            let (s2, t2) = 新编码列表[k + 1];
            if t1 == t2 {
                continue;
            }
            if let Some(_) = difference(&s1, &s2) {
                潜在冲突 += 1;
            }
        }
        (第一冲突, 潜在冲突)
    }
}

// 判断 a 是否是 b 的前缀（允许长度≤4，末尾以 0 填充）
// 注意：此函数不判断严格性：若 a == b 也返回 true
fn difference(a: &Code, b: &Code) -> Option<Code> {
    for i in 0..4 {
        if a[i] == 0 {
            let mut res = [0; 4];
            for j in i..4 {
                res[j - i] = b[j];
            }
            return Some(res); // a 提前结束，是前缀
        }
        if a[i] != b[i] {
            return None;
        }
    }
    Some([0; 4]) // 完整 4 字节相同，也算前缀（但我们会另外排除 a==b）
}
