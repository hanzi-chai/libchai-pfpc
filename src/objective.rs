use std::fmt::Display;

use chai::{
    contexts::default::默认上下文,
    encoders::default::默认编码器,
    objectives::{default::默认目标函数, metric::默认指标, 目标函数},
    元素映射, 编码信息, 错误,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct 冰雪晴芸指标 {
    指标: 默认指标,
    前缀数: usize,
}

impl Display for 冰雪晴芸指标 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "前缀数：{}；{}", self.前缀数, self.指标)
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
        let 前缀数 = self.计算前缀数(&self.目标函数.编码结果);
        let 分数 = 分数 + 前缀数 as f64 * 0.001;
        (冰雪晴芸指标 { 指标, 前缀数 }, 分数)
    }
}

impl 冰雪晴芸目标函数 {
    pub fn 新建(上下文: &默认上下文, 编码器: 默认编码器) -> Result<Self, 错误> {
        Ok(Self {
            目标函数: 默认目标函数::新建(上下文, 编码器)?,
            进制: 上下文.棱镜.进制,
        })
    }

    fn 计算前缀数(&self, 编码结果: &[编码信息]) -> usize {
        let 字符串列表: Vec<[u8; 4]> = 编码结果
            .iter()
            .map(|info| {
                let mut arr = [0u8; 4];
                let mut 编码 = info.全码.原始编码;
                let mut index = 0;
                while 编码 != 0 {
                    arr[index] = (编码 % self.进制) as u8;
                    编码 /= self.进制;
                    index += 1;
                }
                arr
            })
            .collect();
        let 前缀索引 = strict_prefix_indices(&字符串列表);
        let 前缀数 = 前缀索引.len();
        前缀数
    }
}

// 判断 a 是否是 b 的前缀（允许长度≤4，末尾以 0 填充）
// 注意：此函数不判断严格性：若 a == b 也返回 true
fn is_prefix(a: &[u8; 4], b: &[u8; 4]) -> bool {
    for i in 0..4 {
        if a[i] == 0 {
            return true; // a 提前结束，是前缀
        }
        if a[i] != b[i] {
            return false;
        }
    }
    true // 完整 4 字节相同，也算前缀（但我们会另外排除 a==b）
}

pub fn strict_prefix_indices(a: &Vec<[u8; 4]>) -> Vec<usize> {
    let n = a.len();
    let mut indexed: Vec<([u8; 4], usize)> = a.iter().cloned().zip(0..n).collect();
    indexed.sort_unstable_by(|(s1, _), (s2, _)| s1.cmp(s2));
    let mut prefix_indices = Vec::new();
    for k in 0..n - 1 {
        let (ref s1, idx1) = indexed[k];
        let (ref s2, _idx2) = indexed[k + 1];
        if is_prefix(s1, s2) && s1 != s2 {
            prefix_indices.push(idx1);
        }
    }

    prefix_indices
}
