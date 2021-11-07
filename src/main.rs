use std::io;
use std::io::BufRead;

use lindera::tokenizer::Tokenizer;
use lindera_core::LinderaResult;
use regex::Regex;
use sqlite::State;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "NJSLYRust", about = "忍殺語変換")]
struct NJSLYRust {
	/// show part of speech or not 品詞表示
	#[structopt(short, long)]
	partofspeech: bool,

	/// show old text 変換前文字列（解析後）の表示
	#[structopt(short, long)]
	oldtext: bool,

}

struct NinDB {
	name: String,
	value: String,
}

fn avoidast(s: String) -> String
{
	if s == "*" {
		"".to_string()
	} else {
		s
	}
}

fn main() -> LinderaResult<()> {
	let mut nindb: Vec<NinDB> = Vec::new();
	let opt = NJSLYRust::from_args();

	// 変換辞書をsqliteから読みます 埋め込みするにはtemp dbを書き出す必要あり

	let conn = sqlite::open("NJSLYRDict.sqlite").unwrap();
	let mut statement = conn.prepare("SELECT name,value FROM dialect order by id").unwrap();
	while let State::Row = statement.next().unwrap() {
		let unit = NinDB {
			name: statement.read::<String>(0).unwrap(),
			value: statement.read::<String>(1).unwrap(),
		};
		nindb.push(unit);
	}

	// create tokenizer
	let mut tokenizer = Tokenizer::new()?;

	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let text = line.unwrap();
		let tokens = tokenizer.tokenize(&text).unwrap();

		let mut oldstring = String::new();
		// output the tokens
		for token in tokens {
			let mut surv: Vec<String> = Vec::new();
			surv.push(avoidast(token.detail[0].to_string()));
			if token.detail.len() > 1 { // ==1 if UNKNOWN (UNK)
				surv.push(avoidast(token.detail[1].to_string()));
				surv.push(avoidast(token.detail[4].to_string()));
				surv.push(avoidast(token.detail[5].to_string()));
				surv.push(avoidast(token.detail[6].to_string()));
			}
			let text = format!("＜{}＞{}", surv.join(","), token.text);
			oldstring.push_str(&text);
		}
		if opt.oldtext {
			println!("{}\n▼▼変換な▼▼", oldstring);
		}

		// replaceできる限り、忍殺語辞書に基づき変換 正規表現に変更
		for nin in &nindb {
			let re = Regex::new(&nin.name).expect("database item does not follow regexp rule");
			let newstr = re.replace_all(&oldstring, &nin.value);
			oldstring = newstr.to_string();
		}

		// 品詞の＜＞を削除する
		let newstr: String;
		if !opt.partofspeech {
			let re = Regex::new("＜[^＞]+＞").unwrap();
			newstr = re.replace_all(&oldstring, "".to_string()).to_string();
		} else {
			newstr = oldstring;
		}
		println!("{}", newstr);
	}


	Ok(())
}
