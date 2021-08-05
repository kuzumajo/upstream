pub enum StaffTextSize {
  Ultra,
  Large,
  Small,
  Medium,
}

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
    Text("忍野 忍", Medium),
    Text("(姬丝秀忒·雅赛劳拉莉昂·刃下心)", Small),
    Text("八九寺 真宵", Medium),
    Text("神原 骏河", Medium),
    Text("千石 抚子", Medium),
    Text("忍野 扇", Medium),
    Text("老仓 育", Medium),
    Text("忍野 メメ", Medium),
    Text("贝木 泥舟", Medium),
    Text("影缝 余弦", Medium),
    Text("斧乃木 余接", Medium),
    Text("卧烟 伊豆湖", Medium),
    Text("手折 正弦", Medium),
    Text("德拉曼兹路基", Medium),
    Text("艾比所特", Medium),
    Text("奇洛金卡达", Medium),
    Text("死尸累 生死郎", Medium),
    Text("卧烟 远江", Medium),
    Text("(神原 远江)", Small),
    Text("沼地 蜡花", Medium),
    Text("花鸟 楼花", Medium),
    Text("日伞 星雨", Medium),
    Space(480.0),
    Text("DIRECTED BY", Large),
    Space(60.0),
    Text("陈睿 LV6  你所热爱的，就是你的生活。", Medium),
    Text("蒙古上单 LV5  你 妈什么时候死啊？", Medium),
    Space(480.0),
    Text("© KuzumajoStudio", Medium),
    Text("© 屑魔女工作室", Medium),
    Space(480.0),
    Text("av419421212", Small),
  ]
}
