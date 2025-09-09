use std::cmp::Ordering;

pub struct ElementAttribute {
    pub id: i32,
    pub value: i32,
}

pub struct ElementTriggerValue {
    pub value: Vec<i32>,
}

pub struct UnlockCondition {
    pub unlock_type: i32,
    pub condition: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ERaceType {
    None = 0,
    Item = 1,
    Cat = 2,
    Dog = 3,
    Bird = 4,
    Bug = 5,
    Fish = 6,
    Dragon = 7,
    Snake = 8,
    Hoofed = 9,
    Lizard = 10,
    Rodents = 11,
    Molluscs = 12,
    Furry = 13,
    Mod1 = 14,
    Mod2 = 15,
    Mod3 = 16,
    Mod4 = 17,
    Mod5 = 18,
    Mod6 = 19,
    Mod7 = 20,
    Mod8 = 21,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ERare {
    None = 0,
    Common = 1,
    Rare = 2,
    Legend = 3,
    Epic = 4,
    Mythic = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Etip {
    None = 0,
    Sum = 1,
    Before = 2,
    Prob = 3,
    SumEnd = 4,
    Near = 5,
}

pub struct Element {
    /// 这是id
    pub id: i32,
    /// 是否锁定
    pub lock: i32,
    /// 是否启用
    pub enable: i32,
    /// 专属角色
    pub role: i32,
    pub name_id: i32,
    pub desc_id: i32,
    /// 特殊描述类型
    pub desctip: Vec<Etip>,
    /// 描述使用属性
    pub desc_attribute: Vec<i32>,
    /// 全身像
    pub icon: String,
    /// 稀有度
    pub rare: i32,
    /// 种族
    pub race_type: ERaceType,
    /// 其他种族
    pub other_race: ERaceType,
    /// 初始属性
    pub attribute: Vec<ElementAttribute>,
    /// 触发器
    pub trigger_type: i32,
    /// 触发参数
    pub trigger_param: Vec<i32>,
    /// 事件响应语言ID
    pub event_tip: i32,
    /// 触发行为
    pub trigger_action: i32,
    /// 触发行为时数值
    pub trigger_value: Vec<ElementTriggerValue>,
    /// 其他参数
    pub other_value: Vec<i32>,
    /// 攻击音效
    pub attack_sound: i32,
    /// 选中音效
    pub select_sound: i32,
    /// 攻击特效
    pub attack_particle: i32,
}

#[derive(Eq, PartialEq)]
pub struct Enemy {
    /// 这是id
    pub id: i32,
    /// 图标
    pub icon: String,
    /// 多语言ID
    pub name_id: i32,
    /// 描述多语言
    pub desc_id: i32,
    /// 地圖
    pub map: i32,
    /// 关卡
    pub level: i32,
    /// BOSS類型
    pub boss: i32,
    /// 血量
    pub hp: i32,
    /// 攻击力
    pub attack: i32,
    /// 升级攻击力
    pub upgrade_attack: i32,
    /// 血量
    pub endless_hp: i32,
    /// 攻击力
    pub endless_attack: i32,
    /// 升级攻击力
    pub endless_upgrade: i32,
    /// 奖励金币
    pub gold: i32,
    /// 最大奖励金币
    pub gold_max: i32,
    /// 攻击音效
    pub attack_sound: i32,
    /// 出场音效
    pub show_sound: i32,
}

#[derive(Eq, PartialEq)]
pub struct Localization {
    /// 这是id
    pub id: i32,
    pub cn: String,
    pub en: String,
    pub jp: String,
    pub cnt: String,
}

#[derive(Eq, PartialEq)]
pub struct RaceAttribute {
    /// 这是id
    pub id: i32,
    /// 图标
    pub icon: String,
    /// 多语言ID
    pub name_id: i32,
    /// 描述
    pub desc_id: i32,
}

#[derive(Eq, PartialEq)]
pub struct Relics {
    /// 这是id
    pub id: i32,
    /// 是否启用
    pub enable: i32,
    /// 是否锁定
    pub lock: i32,
    /// 专属角色
    pub role: i32,
    /// 教程出现
    pub tutorial: i32,
    /// 图标
    pub icon: String,
    /// 小图标
    pub small_icon: String,
    /// 名称ID
    pub name_id: i32,
    /// 遗物描述ID
    pub desc_id: i32,
    /// 剧情描述ID
    pub other_desc_id: i32,
    /// 特殊描述类型
    pub desc_tip: Vec<Etip>,
    /// 稀有度
    pub rare: i32,
    /// 被动属性
    pub passive: Vec<i32>,
    /// 触发条件
    pub trigger_type: i32,
    /// 触发条件
    pub trigger_param: Vec<i32>,
    /// 事件响应语言ID
    pub event_tip: i32,
    /// 触发行为
    pub trigger_action: i32,
    /// 加成数值
    pub trigger_value: Vec<i32>,
    /// 其他参数
    pub other_value: Vec<i32>,
}

impl Ord for Relics {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Relics {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Element {}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Ord for Enemy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Enemy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Localization {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Localization {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RaceAttribute {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for RaceAttribute {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
