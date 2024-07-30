fn one_step(a: &str, b: &str) -> bool {
  let mut costs = vec![vec![0; b.len() + 1]; a.len() + 1];

  for i in 0..=a.len() {
      costs[i][0] = i;
  }
  for j in 0..=b.len() {
      costs[0][j] = j;
  }

  for (i, ca) in a.chars().enumerate() {
      for (j, cb) in b.chars().enumerate() {
          costs[i + 1][j + 1] = if ca == cb {
              costs[i][j]
          } else {
              1 + std::cmp::min(
                  costs[i][j + 1],
                  std::cmp::min(costs[i + 1][j], costs[i][j]),
              )
          };
      }
  }

  costs[a.len()][b.len()] == 1
}

fn main() {
  // Testes de exemplo
  let str1 = "pale";
  let str2 = "ple";
  println!("Strings estão a uma edição de distância: {}", one_step(str1, str2));

  let str3 = "pales";
  let str4 = "pale";
  println!("Strings estão a uma edição de distância: {}", one_step(str3, str4));

  let str5 = "pale";
  let str6 = "bale";
  println!("Strings estão a uma edição de distância: {}", one_step(str5, str6));

  let str7 = "pale";
  let str8 = "bibo";
  println!("Strings estão a uma edição de distância: {}", one_step(str7, str8));
}