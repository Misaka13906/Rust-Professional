use std::{collections::{HashMap, HashSet}, io::Read};

pub fn count_provinces() -> String {
    let json = read_from_file("./district.json");
    let root_info_vec = parse(json);
    let mut cnt = 0;
    root_info_vec.iter().map(|mp| {
        cnt+=1;
        println!("{cnt}");
        let mut set: HashSet<String> = HashSet::new();
        mp.iter().for_each(|(k, v)| {
            println!("key: {}, value: {}", k, v);
        });
        mp.values().for_each(|x| {
            set.insert(find(&mut mp.clone(), x));
        });
        set.len().to_string()
    }).collect::<Vec<String>>().join(",")
}

fn read_from_file(filename: &str) -> String {
    let mut file = match std::fs::File::open(filename) {
        Err(e) => panic!("无法打开文件: {}", e),
        Ok(f) => f,
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(e) => panic!("读取文件出错: {}", e),
        Ok(_) => (),
    }
    contents
}

fn parse(json: String) -> Vec<HashMap<String, String>> {
    let mut chars: Vec<char> = Vec::new();
    let mut roots: Vec<HashMap<String, String>> = Vec::new();
    let mut tmp_root: HashMap<String, String> = HashMap::new();
    let mut key_name = String::from("");
    let mut tmp_name = String::from("");

    for ch in json.chars() {
        match ch {
            '{' => {
                chars.push(ch);
            },
            '}' => {
                if !(chars.last() == Some(&'{')) {
                    panic!("parse json error: '{{' not found");
                }
                chars.pop();
            },
            '[' => {
                key_name = tmp_name.clone();
                tmp_name = String::from("");
                chars.push('[');
            },
            ']' => {
                if !(chars.last() == Some(&'[')) {
                    let tmp: String = chars.iter().collect();
                    panic!("parse json error: '[' not found, info: {}", tmp);
                }
                chars.pop();
            },
            '"' => {
                if chars.last() != Some(&'"') {
                    chars.push(ch);
                    continue;
                }
                chars.pop();
                if let Ok(index) = tmp_name.parse::<i32>() {
                    tmp_name = String::from("");
                    if index > 1 {
                        roots.push(tmp_root);
                        tmp_root = HashMap::new();
                    }
                    continue;
                }
                if chars.last() == Some(&'[') {
                    merge(&mut tmp_root, &key_name, &tmp_name);
                    tmp_name = String::from("");
                }
            },
            _ => {
                if chars.last() == Some(&'"') {
                    tmp_name.push(ch);
                }
            }
        }
    }
    roots.push(tmp_root);
    roots
}

fn merge(root: &mut HashMap<String, String>, x: &String, y: &String) {
    if root.get(x).is_none() {
        root.insert(x.clone(), x.clone());
    }
    if root.get(y).is_none() {
        root.insert(y.clone(), y.clone());
    }
    let fx = find(root, x);
    let fy = find(root, y);
    if fx != fy {
        root.insert(fx, fy);
    }
}

fn find(root: &mut HashMap<String, String>, x: &String) -> String {
    let mut stack: Vec<String> = Vec::new();
    let mut fx = x.clone();
    while fx != *root.get(&fx).unwrap() {
        stack.push(fx.clone());
        fx = root.get(&fx).unwrap().clone();
    }
    stack.iter().for_each(|x| {
        root.insert(x.clone(), fx.clone());
    });
    fx.clone()
}

// fn find(root: &mut HashMap<String, String>, x: &String) -> String {
//     if root.get(x).unwrap() == x {
//         return x.to_string();
//     }
//     root.insert(x.to_string(), find(root, root.get(x).unwrap()));
//     root.get(x).unwrap().to_string()
// }

/*
int find(int x) {
    if(f[x] == x) {
        return x;
    }
    return f[x] = find(f[x]);
}

int find(int x) {
    stack<int>s;
    while(f[x] != x) {
        s.push(x);
        x = f[x];
    }
    for(int k : s) {
        f[k] = x;
    }
    return x;
}
*/