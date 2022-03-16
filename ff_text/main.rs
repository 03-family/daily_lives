use std::collections::HashMap;

fn main() {
  // https://ja.wikipedia.org/wiki/%E4%B9%9D%E4%B9%9D

  let nums: Vec<usize> = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
  
  let strs = vec!(
    "", "いち", "に", "さん", "し", "ご", "ろく", "しち", 
    "はち", "く", "エー", "ビー", "シー", "ディー", "イー", "エフ",
  );
  let up_strs = vec!(
    "", "いち", "にー", "さん", "よん", "ごー", "ろく", "なな",
    "はち", "きゅう", "エー", "ビー", "シー", "ディー", "イー", "エフ",
  );
  let down_strs = vec!(
    "ぜろ", "いち", "にー", "さん", "よん", "ごー", "ろく", "なな",
    "はち", "きゅう", "エー", "ビー", "シー", "ディー", "イー", "エフ",
  );

  let mut especial_lefts = HashMap::new();
  especial_lefts.insert((2, 2), "ににん");
  especial_lefts.insert((3, 3), "さざん");
  especial_lefts.insert((3, 6), "さぶろく");
  especial_lefts.insert((3, 8), "さんぱ");
  // especial_lefts.insert((4, 8), "しは");
  // especial_lefts.insert((5, 8), "ごは");
  especial_lefts.insert((5, 9), "ごっく");
  // especial_lefts.insert((6, 8), "ろくは");
  especial_lefts.insert((6, 9), "ろっく");
  // especial_lefts.insert((7, 8), "しちは");
  especial_lefts.insert((8, 4), "はっし");
  especial_lefts.insert((8, 8), "はっぱ");
  especial_lefts.insert((8, 9), "はっく");
  especial_lefts.insert((9, 8), "くは");

  let mut texts  = HashMap::<(usize, usize), String>::new();

  for &x in nums.iter() {
    for &y in nums.iter() {
      let ans = x * y;
      // println!("(10進) {x} × {y} = {ans}");
      // println!("(16進) {x:X} × {y:X} = {ans:X}");

      let left = match especial_lefts.get(&(x, y)) {
        Some(&s) => String::from(s),
        None => format!(
          "{}{}",
          if x == 1 { "いん" } else if y > 9 && x == 4 { "よん" } else { strs[x] },
          if x > 9 && y == 4 { "よん" } else if x > 3 && x < 10 && y == 8 { "は" } else { strs[y] },
        )
      };

      let equals = if ans < 10 || x == 1 || y == 1 { " が " } else { " " };

      let ans_up = ans / 16;
      let ans_down = ans % 16;
      let right = format!(
        "{}{}",
        up_strs[ans_up], 
        if ans < 10 { strs[ans_down] } else { down_strs[ans_down] },
      );

      texts.insert((x, y), format!("{left}{equals}{right}"));
    }
  }

  for &x in nums.iter() {
    for &y in nums.iter() {
      println!("{}", texts.get(&(x, y)).unwrap());
    }
  }
}
