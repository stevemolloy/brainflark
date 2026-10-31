/*
Brainflark syntax
>	Move the pointer to the right
<	Move the pointer to the left
+	Increment the memory cell at the pointer
-	Decrement the memory cell at the pointer
.	Output the character signified by the cell at the pointer
,	Input a character and store it in the cell at the pointer
[	Jump past the matching ] if the cell at the pointer is 0
]	Jump back to the matching [ if the cell at the pointer is nonzero
 */

use std::fs;
use std::env;

#[derive(Debug)]
enum Token {
    IncPtr,
    DecrPtr,
    IncrMem,
    DecrMem,
    PutChar,
    GetChar,
    CondJmpFwd(usize),
    CondJmpBck(usize),
    Comment,
}

type Program = Vec<Token>;

struct VM {
    memory: Vec<i32>,
    ptr: usize,
    program: Program,
    program_ptr: usize,
}

impl VM {
    fn from_str(source: &str) -> Self {
	let mut virt_machine: VM = VM {
	    memory: vec![0; MEMSIZE],
	    ptr: MEMSIZE/2,
	    program: vec![],
	    program_ptr: 0,
	};

	virt_machine.compile(&source);

	virt_machine
    }

    fn compile(&mut self, source: &str) {
	let mut jump_stack: Vec<usize> = vec![];

	for (i, c) in source.chars().enumerate() {
	    let token = match c {
		'>' => Token::IncPtr,
		'<' => Token::DecrPtr,
		'+' => Token::IncrMem,
		'-' => Token::DecrMem,
		'.' => Token::PutChar,
		',' => Token::GetChar,
		'[' => {
		    jump_stack.push(i);
		    Token::CondJmpFwd(0)
		}
		']' => {
		    let Some(jmp_ptr) = jump_stack.pop()
		    else {
			eprintln!("ERROR: Cannot find matching '[' to this ']'");
			panic!();
		    };
		    self.program[jmp_ptr] = Token::CondJmpFwd(i+1);
		    Token::CondJmpBck(jmp_ptr)
		}
		_ => Token::Comment,
	    };
	    self.program.push(token);
	}

	if jump_stack.len() > 0 {
	    eprintln!("ERROR: Program contains unclosed '['");
	    panic!();
	}
    }

    fn run(&mut self) {
	while self.program_ptr < self.program.len() {
	    match self.program[self.program_ptr]{
		Token::IncPtr => self.ptr += 1,
		Token::DecrPtr => self.ptr -= 1,
		Token::IncrMem => self.memory[self.ptr] += 1,
		Token::DecrMem => self.memory[self.ptr] -= 1,
		Token::PutChar => print!("{}", self.memory[self.ptr] as u8 as char),
		Token::GetChar => todo!(),
		Token::CondJmpFwd(jmp_to) => {
		    if self.memory[self.ptr] == 0 {
			self.program_ptr = jmp_to - 1;
		    }
		},
		Token::CondJmpBck(jmp_to) => {
		    if self.memory[self.ptr] != 0 {
			self.program_ptr = jmp_to - 1;
		    }
		},
		Token::Comment => (),
	    }
	    self.program_ptr += 1;
	}
    }
}

const MEMSIZE: usize = 1000000;

fn main() {
    let args: Vec<String> = env::args().collect();

    let programname = &args[0];
    if args.len() != 2 {
	eprintln!("ERROR: Please supply a single argument");
	eprintln!("{} <brainflark_file>", programname);
	panic!();
    }
    let filepath = &args[1];
    
    let source = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let mut vm: VM = VM::from_str(&source);
    vm.run();
 }
