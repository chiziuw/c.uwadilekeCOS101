use::std::io::Write;
use::std::io::Read;

fn main() {
    let student_name = ["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
    let matric_number = ["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let department = ["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level = [300, 100, 200, 200, 100];
    let mut file = std::fs::File::create("PAU-SIMS.txt").expect("create failed");
    writeln!(file,"\nstudent_name:{}, matric_number:{}, department:{}, level:{}",student_name[0],matric_number[0],department[0],level[0]);
    writeln!(file,"\nstudent_name:{}, matric_number:{}, department:{}, level:{}",student_name[1],matric_number[1],department[1],level[1]);
    writeln!(file,"\nstudent_name:{}, matric_number:{}, department:{}, level:{}",student_name[2],matric_number[2],department[2],level[2]);
    writeln!(file,"\nstudent_name:{}, matric_number:{}, department:{}, level:{}",student_name[3],matric_number[3],department[3],level[3]);
    writeln!(file,"\nstudent_name:{}, matric_number:{}, department:{}, level:{}",student_name[4],matric_number[4],department[4],level[4]);
}