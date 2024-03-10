use std::{
    fs, io,
    fs::File,
    io::{BufReader,BufRead,Write},
    path::{Path, PathBuf},
};
// :TODO slide filename into header

fn main() {
    let csv_path = "c:/Users/ozino/OneDrive/Desktop/test.csv";
    // let path ="C:/Users/ozino/OneDrive/Documents/Program/test_ref/";
    println!("Input directory path.:");
    let path = std_input();
    println!("Input column number which you want to reflect.Default is [1 2]");
    let mut num_col:[usize;2] = [1,2];
    let numbers_str = std_input();
    num_col[0] = numbers_str.split(" ").collect::<Vec<_>>()[0].parse().unwrap();
    num_col[1] = numbers_str.split(" ").collect::<Vec<_>>()[1].parse().unwrap();


    // get the contents in the directory
    let mut files :Vec<PathBuf> =Vec::new();
    match read_dir_entries(&path) {
        Ok(entries) => files = entries,
        Err(e) => eprintln!("ERROR: {}", e),
    }
    // open and push to vector from only csv file text
    let mut yet_concatenated:Vec<Vec<String>> = Vec::new();
    for f in &files{
        if f.clone().into_os_string().into_string().unwrap().contains(".csv"){
            let text :Vec<String> = import_text(&f).unwrap();
            // let concatenated_aline =  
            // println!("{:?}", text);
            yet_concatenated.push(text);
        }
    }
    // limit the number of columns to 2.
    let yet_concatenated = format_data(yet_concatenated, num_col);
    // concatenate texts
    let mut concatenated_text: Vec<String> = Vec::new();
    for ii in 0..max_length(&yet_concatenated){
        let mut aline = String::new();
        for text in &yet_concatenated{
            if ii < text.len(){
                aline = format!("{}{},,",aline,text[ii]);
            }else{
                aline = format!("{},,,,",aline)
            }
        }
        concatenated_text.push(aline);
    }
    println!("{:?}", concatenated_text);
    
    let data =format!("{}\n{}", get_filename_header(files), cast_to_str(concatenated_text));
    write_csv(csv_path, &data);
}


fn get_filename_header(files:Vec<PathBuf>)->String{
    let mut columns :String= String::new();
    for f in files{
        let file_name_str = f.clone().into_os_string().into_string().unwrap();
        if file_name_str.contains(".csv"){
            columns = format!("{}{},,,", columns, file_name_str);
        }
    }
    columns
}

fn format_data(files:Vec<Vec<String>>, num_col:[usize;2])->Vec<Vec<String>>{
    let mut result:Vec<Vec<String>> = Vec::new();
    for f in files{
        let f = skip_index(f);
        let a_file = remove_columns(f,num_col);
        result.push(a_file); 
    }
    result 
}

fn skip_index(file:Vec<String>) ->Vec<String>{
    let mut result :Vec<String> = Vec::new();
    let passed:bool = false;
    let keyword = "index";
    for ii in file{
        if ii.contains(keyword){
            let passed = true;
        }
        if passed{
            result.push(ii);
        }
    }
    result
}

fn remove_columns(file:Vec<String>, num_col:[usize;2])->Vec<String>{
    let mut result:Vec<String> = Vec::new();
    for ii in file{
        let comma_splitted =ii.split(",").collect::<Vec<_>>();
        // println!("{:?}", comma_splitted);
        result.push(format!("{},{}",comma_splitted[num_col[0]], comma_splitted[num_col[1]]));
    }
    result
}

fn write_csv(path: &str, data:&String){ //-> std::io::Result<()> {
    let file = File::create(path);
    writeln!(file.unwrap(),"{}",data).unwrap();
}

fn cast_to_str(text:Vec<String>)->String{
    let mut result = String::new();
    for t in text{
        result = format!("{}{}\n",result,t)
    }
    result
}

fn max_length(vecvec : &Vec<Vec<String>>) -> usize{
    let mut max = 0;
    for t in vecvec{
        if max < t.len(){
            max = t.len();
        }
    }
    max
}

fn std_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().to_string()
}

fn import_text(file_path: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut lin
    : Vec<String> = Vec::new();
    for result in BufReader::new(File::open(file_path)?).lines() {
        let l = result?;
        // println!("{}", l);
        lin.push(l)
    }
    Ok(lin)
}

fn read_dir_entries<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();
    Ok(entries)
}
