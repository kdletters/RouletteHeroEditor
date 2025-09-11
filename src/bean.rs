use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ffi::CString;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ElementAttribute {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Value")]
    pub value: i32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ElementTriggerValue {
    #[serde(rename = "Value")]
    pub value: Vec<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UnlockCondition {
    #[serde(rename = "UnlockType")]
    pub unlock_type: i32,
    #[serde(rename = "Condition")]
    pub condition: i32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ModData
{
    #[serde(rename = "ModName")]
    pub name: String,
    #[serde(rename = "ModDesc")]
    pub desc: String,
    #[serde(rename = "EntityMod")]
    pub entity : bool,
    #[serde(rename = "EnemyMod")]
    pub enemy: bool,
    #[serde(rename = "Version")]
    pub version : String,
    #[serde(rename = "CardMod")]
    pub card : bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ERaceType {
    #[default]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ERare {
    #[default]
    None = 0,
    Common = 1,
    Rare = 2,
    Legend = 3,
    Epic = 4,
    Mythic = 5,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Etip {
    #[default]
    None = 0,
    Sum = 1,
    Before = 2,
    Prob = 3,
    SumEnd = 4,
    Near = 5,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Element {
    /// 这是id
    #[serde(rename = "Id")]
    pub id: i32,
    /// 是否锁定
    #[serde(rename = "Lock")]
    pub lock: i32,
    /// 是否启用
    #[serde(rename = "Enable")]
    pub enable: i32,
    /// 专属角色
    #[serde(rename = "Role")]
    pub role: i32,
    #[serde(rename = "NameId")]
    pub name_id: i32,
    #[serde(rename = "DescId")]
    pub desc_id: i32,
    /// 特殊描述类型
    #[serde(rename = "Desctip")]
    pub desctip: Vec<Etip>,
    /// 描述使用属性
    #[serde(rename = "DescAttribute")]
    pub desc_attribute: Vec<i32>,
    /// 全身像
    #[serde(rename = "Icon")]
    pub icon: String,
    /// 稀有度
    #[serde(rename = "Rare")]
    pub rare: i32,
    /// 种族
    #[serde(rename = "RaceType")]
    pub race_type: ERaceType,
    /// 其他种族
    #[serde(rename = "OtherRace")]
    pub other_race: ERaceType,
    /// 初始属性
    #[serde(rename = "Attribute")]
    pub attribute: Vec<ElementAttribute>,
    /// 触发器
    #[serde(rename = "TriggerType")]
    pub trigger_type: i32,
    /// 触发参数
    #[serde(rename = "TriggerParam")]
    pub trigger_param: Vec<i32>,
    /// 事件响应语言ID
    #[serde(rename = "EventTip")]
    pub event_tip: i32,
    /// 触发行为
    #[serde(rename = "TriggerAction")]
    pub trigger_action: i32,
    /// 触发行为时数值
    #[serde(rename = "TriggerValue")]
    pub trigger_value: Vec<ElementTriggerValue>,
    /// 其他参数
    #[serde(rename = "OtherValue")]
    pub other_value: Vec<i32>,
    /// 攻击音效
    #[serde(rename = "AttackSound")]
    pub attack_sound: i32,
    /// 选中音效
    #[serde(rename = "SelectSound")]
    pub select_sound: i32,
    /// 攻击特效
    #[serde(rename = "AttackParticle")]
    pub attack_particle: i32,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Enemy {
    /// 这是id
    #[serde(rename = "Id")]
    pub id: i32,
    /// 图标
    #[serde(rename = "Icon")]
    pub icon: String,
    /// 多语言ID
    #[serde(rename = "NameId")]
    pub name_id: i32,
    /// 描述多语言
    #[serde(rename = "DescId")]
    pub desc_id: i32,
    /// 地圖
    #[serde(rename = "Map")]
    pub map: i32,
    /// 关卡
    #[serde(rename = "Level")]
    pub level: i32,
    /// BOSS類型
    #[serde(rename = "Boss")]
    pub boss: i32,
    /// 血量
    #[serde(rename = "Hp")]
    pub hp: i32,
    /// 攻击力
    #[serde(rename = "Attack")]
    pub attack: i32,
    /// 升级攻击力
    #[serde(rename = "UpgradeAttack")]
    pub upgrade_attack: i32,
    /// 血量
    #[serde(rename = "EndlessHp")]
    pub endless_hp: i32,
    /// 攻击力
    #[serde(rename = "EndlessAttack")]
    pub endless_attack: i32,
    /// 升级攻击力
    #[serde(rename = "EndlessUpgrade")]
    pub endless_upgrade: i32,
    /// 奖励金币
    #[serde(rename = "Gold")]
    pub gold: i32,
    /// 最大奖励金币
    #[serde(rename = "GoldMax")]
    pub gold_max: i32,
    /// 攻击音效
    #[serde(rename = "AttackSound")]
    pub attack_sound: i32,
    /// 出场音效
    #[serde(rename = "ShowSound")]
    pub show_sound: i32,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Localization {
    /// 这是id
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Cn")]
    pub cn: String,
    #[serde(rename = "En")]
    pub en: String,
    #[serde(rename = "Jp")]
    pub jp: String,
    #[serde(rename = "Cnt")]
    pub cnt: String,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RaceAttribute {
    /// 这是id
    #[serde(rename = "Id")]
    pub id: i32,
    /// 图标
    #[serde(rename = "Icon")]
    pub icon: String,
    /// 多语言ID
    #[serde(rename = "NameID")]
    pub name_id: i32,
    /// 描述
    #[serde(rename = "DescID")]
    pub desc_id: i32,
}

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Relics {
    /// 这是id
    #[serde(rename = "Id")]
    pub id: i32,
    /// 是否启用
    #[serde(rename = "Enable")]
    pub enable: i32,
    /// 是否锁定
    #[serde(rename = "Lock")]
    pub lock: i32,
    /// 专属角色
    #[serde(rename = "Role")]
    pub role: i32,
    /// 教程出现
    #[serde(rename = "Tutorial")]
    pub tutorial: i32,
    /// 图标
    #[serde(rename = "Icon")]
    pub icon: String,
    /// 小图标
    #[serde(rename = "SmallIcon")]
    pub small_icon: String,
    /// 名称ID
    #[serde(rename = "NameId")]
    pub name_id: i32,
    /// 遗物描述ID
    #[serde(rename = "DescId")]
    pub desc_id: i32,
    /// 剧情描述ID
    #[serde(rename = "OtherDescId")]
    pub other_desc_id: i32,
    /// 特殊描述类型
    #[serde(rename = "DescTip")]
    pub desc_tip: Vec<Etip>,
    /// 稀有度
    #[serde(rename = "Rare")]
    pub rare: i32,
    /// 被动属性
    #[serde(rename = "Passive")]
    pub passive: Vec<i32>,
    /// 触发条件
    #[serde(rename = "TriggerType")]
    pub trigger_type: i32,
    /// 触发条件
    #[serde(rename = "TriggerParam")]
    pub trigger_param: Vec<i32>,
    /// 事件响应语言ID
    #[serde(rename = "EventTip")]
    pub event_tip: i32,
    /// 触发行为
    #[serde(rename = "TriggerAction")]
    pub trigger_action: i32,
    /// 加成数值
    #[serde(rename = "TriggerValue")]
    pub trigger_value: Vec<i32>,
    /// 其他参数
    #[serde(rename = "OtherValue")]
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
