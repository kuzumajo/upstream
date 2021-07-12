
#[derive(Clone)]
pub enum StaffTextSize {
  Ultra,
  Large,
  Small,
  Medium,
}

#[derive(Clone)]
pub enum StaffTextLine {
  Text(&'static str, StaffTextSize),
  Space(f32),
}

pub fn get_staff_text() -> Vec<StaffTextLine> {
  use StaffTextLine::*;
  use StaffTextSize::*;
  vec![
    Text("UPSTREAM", Ultra),
    Text("溯流而上", Ultra),
    Space(480.0),
    Text("STAFF", Large),
    Space(60.0),
    Text("イレイナ - 本渡楓", Medium),
    Text("サヤ - 黒沢ともよ", Medium),
    Text("フラン - 花澤香菜", Medium),
    Space(180.0),
    Text("SPECIAL THANKS", Large),
    Space(60.0),
    Text("和泉 纱雾", Medium),
    Text("山田 妖精", Medium),
    Text("高坂 桐乃", Medium),
    Text("五更 琉璃", Medium),
    Text("新垣 绫濑", Medium),
    Text("来栖 加奈子", Medium),
    Text("阿库娅", Medium),
    Text("惠惠", Medium),
    Text("达克妮斯", Medium),
    Text("鹿目 圆", Medium),
    Text("暁美 焰", Medium),
    Text("美树 沙耶香", Medium),
    Text("巴麻美", Medium),
    Text("佐仓 杏子", Medium),
    Text("阿良良木 历", Medium),
    Text("阿良良木 火怜", Medium),
    Text("阿良良木 月火", Medium),
    Text("战场原 黑仪", Medium),
    Text("羽川 翼", Medium),
    Text("忍野 忍(姬丝秀忒·雅赛劳拉莉昂·刃下心)", Medium),
    Text("八九寺 真宵", Medium),
    Text("神原 骏河", Medium),
    Text("千石 抚子", Medium),
    Text("忍野 扇", Medium),
    Text("老仓 育", Medium),
    Text("广", Medium),
    Text("02", Medium),
    Text("莓", Medium),
    Text("五郎", Medium),
    Text("未来", Medium),
    Text("纯位数", Medium),
    Text("心", Medium),
    Text("满", Medium),
    Text("郁乃", Medium),
    Text("太", Medium),
    Space(480.0),
    Text("© KuzumajoStudio", Medium),
    Text("© 屑魔女工作室", Medium),
  ]
}
