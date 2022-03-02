use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::time::Instant;

static LUA_HEADER :&str = "\
--Code generated by luna's epic isEven function\n
--Learn more at https://raw.githubusercontent.com/pozm/lua-isEven/master/cry.png\n
function isEven(number)
";
static  LUA_FOOTER :&str = "\

end";
fn main() {
    let now = Instant::now();
    let mut bytes = Vec::<u8>::from(LUA_HEADER);
    println!("to what number do i generate?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let mut f = File::create("./funny.lua").unwrap();
    let num = u64::from_str(input.as_str().trim_end()).unwrap_or(1);
    f.write_all(&*bytes);
    bytes.clear();
    for i in 0..num {
        bytes.append(&mut Vec::<u8>::from(format!("if (number == {}) then return {} end\n",i,i%2==0)));
        if i % 10000 == 0 {
            f.write(&mut bytes);
            bytes.clear();
        }
    }
    bytes.append(&mut Vec::<u8>::from(format!("{}\n -- generated {} in {:?} | lol",LUA_FOOTER,num,now.elapsed())));
    f.write(&*bytes);
    // println!("{:#?}",bytes)
}
