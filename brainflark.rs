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
    memory: Vec<u32>,
    ptr: usize,
    program: Program,
    program_ptr: usize,
}

impl VM {
    fn run(&mut self) {
	while self.program_ptr < self.program.len() {
	    match self.program[self.program_ptr]{
		Token::IncPtr => {
		    self.ptr += 1;
		    self.program_ptr += 1;
		},
		Token::DecrPtr => {
		    self.ptr -= 1;
		    self.program_ptr += 1;
		},
		Token::IncrMem => {
		    self.memory[self.ptr] += 1;
		    self.program_ptr += 1;
		},
		Token::DecrMem => {
		    self.memory[self.ptr] -= 1;
		    self.program_ptr += 1;
		},
		Token::PutChar => {
		    print!("{}", self.memory[self.ptr] as u8 as char);
		    self.program_ptr += 1;
		},
		Token::GetChar => todo!(),
		Token::CondJmpFwd(jmp_to) => {
		    if self.memory[self.ptr] == 0 {
			self.program_ptr = jmp_to;
		    } else {
			self.program_ptr += 1;
		    }
		},
		Token::CondJmpBck(jmp_to) => {
		    if self.memory[self.ptr] == 0 {
			self.program_ptr += 1;
		    } else {
			self.program_ptr = jmp_to;
		    }
		},
		Token::Comment => todo!(),
	    }
	}
    }
}

fn compile(source: &str) -> Program {
    let mut program: Program = vec![];
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
		program[jmp_ptr] = Token::CondJmpFwd(i+1);
		Token::CondJmpBck(jmp_ptr)
	    }
	    _ => Token::Comment,
	};
	program.push(token);
    }
    program
}

fn main() {
    let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string();

    let program: Program = compile(&source);

    let mut vm: VM = VM {
	memory: vec![0; 1000000],
	ptr: 500000,
	program,
	program_ptr: 0,
    };

    vm.run();
 }
