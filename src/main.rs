//use std::io;
use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = parse_config(&args);

    //println!("Input File {}",filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    println!("{}",start_assembler(&contents));

}

pub fn start_assembler(contents: &str) -> String{
    let mut out_contents: String = "".to_owned();
    let mut var_list = vec![];
    let mut jump_map: HashMap::<String, i16> = HashMap::new();
    let mut cur_line: i16 = 0;

    for line in contents.lines(){
        let mut out_line = line.trim().to_string();
        out_line = remove_comments(&out_line);
        if !out_line.contains("(") && out_line != "" {
            out_line = to_machine(out_line , &mut var_list, &jump_map);
            out_contents.push_str(&out_line);
            out_contents.push_str("\n");
            cur_line +=1;
        }else if out_line.contains("("){
            let out_line = out_line;

            jump_map.insert(out_line.replace("(","").replace(")",""), cur_line);
            
        }
    }
    out_contents
}

fn parse_config(args: &[String]) -> &str {

    if args.len() < 2 {
        panic!("Missing Arguments! 😢");
    }

    let filename = &args[1];

    filename
}

fn remove_comments(line: &str) -> String {
    let mut out_str: String = String::from("");
    for c in line.chars(){
        if c == '/'{
            break;
        }
        out_str.push(c);
    }

    out_str
}

fn compare_comp(comp: &str) -> String{
    let comp_out: String;
    match comp.trim() {
        "0"=> comp_out = "0101010".to_string(),
        "1"=> comp_out = "0111111".to_string(),
        "-1"=> comp_out = "0111010".to_string(),
        "D"=> comp_out = "0001100".to_string(),
        "A"=> comp_out = "0110000".to_string(),
        "!D"=> comp_out = "0001101".to_string(),
        "!A"=> comp_out = "0110001".to_string(),
        "-D"=> comp_out = "0011111".to_string(),
        "-A"=> comp_out = "0110011".to_string(),
        "D+1"=> comp_out = "0011111".to_string(),
        "A+1"=> comp_out = "0110111".to_string(),
        "D-1"=> comp_out = "0001110".to_string(),
        "A-1"=> comp_out = "0110010".to_string(),
        "D+A"=> comp_out = "0000010".to_string(),
        "D-A"=> comp_out = "0010011".to_string(),
        "A-D"=> comp_out = "0000111".to_string(),
        "D&A"=> comp_out = "0000000".to_string(),
        "D|A"=> comp_out = "0010101".to_string(),
        "M"=> comp_out = "1110000".to_string(),
        "!M"=> comp_out = "1110001".to_string(),
        "-M"=> comp_out = "1110011".to_string(),
        "M+1"=> comp_out = "1110111".to_string(),
        "M-1"=> comp_out = "1110010".to_string(),
        "D+M"=> comp_out = "1000010".to_string(),
        "D-M"=> comp_out = "1010011".to_string(),
        "M-D"=> comp_out = "1000111".to_string(),
        "D&M"=> comp_out = "1000000".to_string(),
        "D|M"=> comp_out = "1010101".to_string(),
        _=> comp_out = "xxxxxxx".to_string(),
    }
    comp_out
}

fn to_machine(line: String ,var_list: &mut Vec<String>, jump_map: &HashMap<String, i16>) -> String {
    let mut out_str: String;
    //If A instruction
    if line.contains('@'){
        let line = line.replace("@","");
        let instruction = "0".to_owned();
        
        if line.chars().nth(0).unwrap().is_numeric() {
            out_str = instruction + &format!("{:015b}",line.parse::<i16>().unwrap());
        }else{
            println!("{}",line);
            match line.trim() {
                "SP"=> out_str = "0000000000000000".to_string(),
                "R0"=> out_str = "0000000000000000".to_string(),
                "LCL"=> out_str = "0000000000000001".to_string(),
                "R1"=> out_str = "0000000000000001".to_string(),
                "ARG"=> out_str = "0000000000000010".to_string(),
                "R2"=> out_str = "0000000000000010".to_string(),
                "THIS"=> out_str = "0000000000000011".to_string(),
                "R3"=> out_str = "0000000000000011".to_string(),
                "THAT"=> out_str = "0000000000000100".to_string(),
                "R4"=> out_str = "0000000000000100".to_string(),
                "R5"=> out_str = "0000000000000101".to_string(),
                "R6"=> out_str = "0000000000000110".to_string(),
                "R7"=> out_str = "0000000000000111".to_string(),
                "R8"=> out_str = "0000000000001000".to_string(),
                "R9"=> out_str = "0000000000001001".to_string(),
                "R10"=> out_str = "0000000000001010".to_string(),
                "R11"=> out_str = "0000000000001011".to_string(),
                "R12"=> out_str = "0000000000001100".to_string(),
                "R13"=> out_str = "0000000000001101".to_string(),
                "R14"=> out_str = "0000000000001110".to_string(),
                "R15"=> out_str = "0000000000001111".to_string(),
                "SCREEN"=> out_str = "0100000000000000".to_string(),
                "KDB"=> out_str = "0110000000000000".to_string(),
                _=> out_str = "".to_string(),
            }
            if out_str == ""{
                if var_list.contains(&line){
                    let index_element = var_list.iter()
                                                .position(|x| x == &line)
                                                .unwrap();
        
                    out_str = instruction + &format!("{:015b}",index_element + 16);
                }else if jump_map.contains_key(&line as &str) {
                    match jump_map.get(&line as &str){
                        Some(loc) => out_str = instruction + &format!("{:015b}",loc),
                        None=> panic!("Jump_Map error"),
                    }
                    
                }else{
                    out_str = instruction + &format!("{:015b}",var_list.len() + 16);
                    var_list.push(line);
                }
            }
        } 
    }else{
        let instruction = "111".to_owned();
        let comp: String;
        let mut jump: String = String::from("000");
        let mut dest: String = String::from("000");
        //If C instruction
        // This *Might* not work
        if line.contains(';'){
            //If JUMP
            let asign: Vec<&str> = line.split(";").collect();
            match asign[1].trim() {
                "JGT"=> jump = "001".to_string(),
                "JEQ"=> jump = "010".to_string(),
                "JGE"=> jump = "011".to_string(),
                "JLT"=> jump = "100".to_string(),
                "JNE"=> jump = "101".to_string(),
                "JLE"=> jump = "110".to_string(),
                "JMP"=> jump = "111".to_string(),
                _=> jump = "xxx".to_string(),
            }
            comp = compare_comp(asign[0]);

        }else{
            let asign: Vec<&str> = line.split("=").collect();
            match asign[0].trim() {
                "M"=> dest = "001".to_string(),
                "D"=> dest = "010".to_string(),
                "MD"=> dest = "011".to_string(),
                "A"=> dest = "100".to_string(),
                "AM"=> dest = "101".to_string(),
                "AD"=> dest = "110".to_string(),
                "AMD"=> dest = "111".to_string(),
                _=> dest = "xxx".to_string(),
            }
            comp = compare_comp(asign[1]);
        }

        out_str = instruction + &comp + &dest + &jump;
       
    }

    out_str
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn no_variables() {
        let contents = "// Adds 1 + ... + 100
        M=1 // i=1
        M=0 // sum=0
        (LOOP)
        D=M // D=i
        D=D-A // D=i-100
        D;JGT // if (i-100)>0 goto END
        D=M // D=i
        M=D+M // sum=sum+i
        M=M+1 // i=i+1
        0;JMP // goto LOOP
        (END)
        0;JMP // infinite loop";

        assert_eq!("1110111111001000
1110101010001000
1111110000010000
1110010011010000
1110001100000001
1111110000010000
1111000010001000
1111110111001000
1110101010000111
1110101010000111
", start_assembler(contents));
    }

    #[test]
    fn comments() {
        let contents = "// Adds 1 + ... + 100
        M=1 // i=1";

        assert_eq!("1110111111001000
", start_assembler(contents));
    }

    #[test]
    fn with_variables() {
        let contents = "// Adds 1 + ... + 100
        @i
        M=1 // i=1
        @sum
        M=0 // sum=0
        (LOOP)
        @i
        D=M // D=i
        @100
        D=D-A // D=i-100
        @END
        D;JGT // if (i-100)>0 goto END
        @i
        D=M // D=i
        @sum
        M=D+M // sum=sum+i
        @i
        M=M+1 // i=i+1
        @LOOP
        0;JMP // goto LOOP
        (END)
        @END
        0;JMP // infinite loop";

        assert_eq!("0000000000010000
1110111111001000
0000000000010001
1110101010001000
0000000000010000
1111110000010000
0000000001100100
1110010011010000
0000000000010010
1110001100000001
0000000000010000
1111110000010000
0000000000010001
1111000010001000
0000000000010000
1111110111001000
0000000000000100
1110101010000111
0000000000010010
1110101010000111
", start_assembler(contents));
    }

    #[test]
    fn predefined_symbols() {
    let contents = "@R1
    @R2
    @SP
    @LCL
    @SCREEN
    ";

    assert_eq!("0000000000000001
0000000000000010
0000000000000000
0000000000000001
0100000000000000
", start_assembler(contents));
    }
}
