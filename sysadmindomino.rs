#[derive(Debug)]
pub struct Tile {
    a: i32,
    b: i32,
}

impl Tile {
    pub fn new(x: i32, y: i32) -> Self {

        let t = Tile { 
            a: x,
            b: y, 
        };

        t

    }

    pub fn print(&self) {

     	print!("[{}|{}]", self.a, self.b);

    }
}

#[derive(Debug)]
pub struct Player {
    name: String,
    pile: Vec<Tile>,
}

impl Player {
    pub fn new(n: &str, by: &mut Vec<Tile>) -> Self {

    	let mut i = 0;
    	let mut p = Vec::new();
    	let mut l = n.len();

    	if l > 7 {

    		l = l%7;

    	}

    	let mut index = 0;

    	while i < 7 {

   	     	index = index + l;

   	     	if index > by.len() - 1 {

   	     		index = index%by.len() - 1;

   	     	}

			p.push(by.remove(index));

			i = i + 1;

    	}

        let player = Player { 
            name: n.to_string(),
            pile: p, 
        };

        player

    }

    pub fn draw_tile(&mut self, by: &mut Vec<Tile>){

    	if by.len() > 1 {

    		let mut index = self.name.len();

    		if index > 7 {

    			index = index%7;

    		}

    		let size = by.len();

   	   		if index > size - 1 {

   	   			index = index%size - 1;

   	   		}

   	   		&self.pile.push(by.remove(index));

   	   	}else if by.len() == 1{

   	   		&self.pile.push(by.remove(0));

   	   	}else{

			println!("Boneyard in empty!");

		}

    }

    pub fn print(&self) {

    	println!("Now playing: {}", self.name);

    	let mut n = 1;

		for i in &self.pile {

			print!(" {}.", n);
    		i.print();

    		n = n + 1;

		}

		println!();

    }
}

use std::io::{self};
use std::{thread, time};

#[derive(Debug)]
pub struct Game {
    player1: Player,
    player2: Player,
    boneyard: Vec<Tile>,
	openends: Vec<i32>,
	last: i32,
}

impl Game {
    pub fn new() -> Self {

    	let mut b = Vec::new();
    	let mut k = 0;
    	let mut i = 0;
    	let mut j = 0;

    	while k < 28 {

    		let t = Tile::new(i,j);
    		b.push(t);

    		k = k + 1;

    		if i==j {

    			i = 0;
    			j = j+1;

    		}else {

    			i = i + 1;

    		}

    	}	

       	println!("////////////////////////////////////////////////////////////////////////////////
////////////////////////////////   Welcome to:   ///////////////////////////////
////////////////////   [-S-Y-S-A-D-M-I-N-] DRAW DOMINOES   /////////////////////
////////////////////////////////////////////////////////////////////////////////
(*) Instructions:
$> [ -p ] - command for picking a tile to make your 
move with and you'll be asked to pick it's subsequent 
number in your pile
$> [ -ne ] - command for displaying another open end
$> [ -dp ] - command for drawing another tile from 
the boneyard
$> [ -pm ] - command for passing a move
$> [ -q ] - command for quiting the game");

       	println!();println!();

    	let mut name1 = String::new();

    	println!("dominogame: ~ $ Name of Player1:");

    	match io::stdin().read_line(&mut name1) {
        	Err(e) => panic!("couldn't read the name of player1: {}", e),
       		Ok(f) => f,
    	};

    	let mut name2 = String::new();

    	println!("dominogame: ~ $ Name of Player2:");

    	match io::stdin().read_line(&mut name2) {
        	Err(e) => panic!("couldn't read the name of player2: {}", e),
       		Ok(f) => f,
    	};

    	let p1 = Player::new(&name1.trim(),&mut b);
    	let p2 = Player::new(&name2.trim(),&mut b);

    	let g = Game { 
            player1: p1,
            player2: p2,
            boneyard: b, 
			openends: Vec::new(),
			last: 7,
		};

        g

    }

    fn graphics(&self, which: i32){
    	match which {
    		0 => println!("_______________
|               |
|               |
|               |
|               |
|               |
|               |
|_______________|"),
    		1 => println!("_______________
|               |
|               |
|       _       |
|      |_|      |
|               |
|               |
|_______________|"),
    		2 => println!("_______________
|            _  |
|           |_| |
|               |
|               |
|  _            |
| |_|           |
|_______________|"),
    		3 => println!("_______________
|            _  |
|           |_| |
|       _       |
|      |_|      |
|  _            |
| |_|           |
|_______________|"),
    		4 => println!("_______________
|  _         _  |
| |_|       |_| |
|               |
|               |
|  _         _  |
| |_|       |_| |
|_______________|"),
    		5 => println!("_______________
|  _         _  |
| |_|       |_| |
|       _       |
|      |_|      |
|  _         _  |
| |_|       |_| |
|_______________|"),
    		6 => println!("_______________
|  _         _  |
| |_|       |_| |
|  _         _  |
| |_|       |_| |
|  _         _  |
| |_|       |_| |
|_______________|"),
    		_ => println!("\n\n\n\n\n\n\n\n"),
    	}
    }

    fn pick_loop(&mut self, cur: i32) -> bool {

    	let mut pickedmove: bool = false;
    	let mut quit: bool = false;
    	let mut first: bool = false;

    	while !pickedmove && !quit {

    		if self.openends.len() == 0 {

    			&self.graphics(7);
    			&self.graphics(7);

    		}

    		if cur == 1 {

    			self.player1.print();

    		}else{

				self.player2.print();

    		}

    		let mut command = String::new();

    		println!("dominogame: ~ $ Pick:");

    		match io::stdin().read_line(&mut command) {
        		Err(e) => panic!("couldn't read the command: {}", e),
       			Ok(f) => f,
   			};

   			if command.trim() == "-pm" {

    			&self.graphics(self.last);

   			}else if command.trim() == "-ne"{

    			let mut iter = self.openends.iter();
    			let mut again: bool = true;
    			let mut c = 1;
    			let len = self.openends.len();

    			while again {

    				let nextone = iter.next();

    				if nextone == None {

    					iter = self.openends.iter();
    					self.last = *iter.next().unwrap();
    					c = 1;

    				}else{

    					self.last = *nextone.unwrap();

    				}

    				&self.graphics(self.last);
    				&self.graphics(7);

    				println!("Open ends: {} (out of {})", c, len);

    				if cur == 1 {

    					self.player1.print();

    				}else{

						self.player2.print();

    				}

    				println!("dominogame: ~ $ Again?([-y] - Yes, [-n] - No):");

    				let mut choice = String::new();

    				match io::stdin().read_line(&mut choice) {
        				Err(e) => panic!("couldn't read the command: {}", e),
       					Ok(f) => f,
   					};

   					if choice.trim() == "-n" {

   						again = false;
   						&self.graphics(self.last);

   					}else{

   						c = c + 1;

   					}

   				}
    		}else if command.trim() == "-dp"{

    			if cur == 1 {

    				self.player1.draw_tile(&mut self.boneyard);

    			}else{

					self.player2.draw_tile(&mut self.boneyard);

    			}

    			&self.graphics(self.last);

   			}else if command.trim() == "-p" {

    			println!("dominogame: ~ $ Pick:");

    			let mut num = String::new();

    			match io::stdin().read_line(&mut num) {
        			Err(e) => panic!("couldn't read the number: {}", e),
       				Ok(f) => f,
   				};


   				let mut chosen = num.trim().chars().next().unwrap().to_digit(10).unwrap() as usize;
   				if num.trim().len()>1 {

   					chosen = 10*chosen + num.trim().chars().next().unwrap().to_digit(10).unwrap() as usize;

   				}

   				&self.graphics(self.last);

   				if (chosen <= self.player1.pile.len() && cur == 1) ||
   				   (chosen <= self.player2.pile.len() && cur == 2) {

	    			if self.openends.len() > 0 {

	    				let mut index = 0;

	    				for l in &self.openends {

	    					if l == &self.last {
	    						break;
	    					}	

	    					index = index + 1;

	    				}

	    				if index < self.openends.len() {

	    					&self.openends.remove(index);

	    				}

	    			}else{

	    				first = true;

	    			}

	    			if cur == 1 {

	    				let picked = self.player1.pile.remove(chosen-1);

	    				if picked.a != self.last {

	    					&self.graphics(picked.b);
	    					&self.graphics(picked.a);
	    					self.last = picked.a;

	    				}else {

	    					&self.graphics(picked.a);
	    					&self.graphics(picked.b);
	    					self.last = picked.b;

	    				}

	    				if picked.a == picked.b || first {

	    					&self.openends.push(picked.a);
	    					&self.openends.push(picked.b);

	    				}

	    				if picked.a != self.last && picked.b != self.last {

	    					println!("Wrong move, but your funeral...");

	    				}

	    			}else{

	    				let picked = self.player2.pile.remove(chosen-1);

						if picked.a != self.last {

	    					&self.graphics(picked.b);
	    					&self.graphics(picked.a);
	    					self.last = picked.a;

	    				}else {

	    					&self.graphics(picked.a);
	    					&self.graphics(picked.b);
	    					self.last = picked.b;

	    				}

	    				if picked.a == picked.b || first {

	    					&self.openends.push(picked.a);
	    					&self.openends.push(picked.b);

	    				}

	    				if picked.a != self.last && picked.b != self.last {

	    					println!("Wrong move, but your funeral...");

	    				}

	    			}

	    			if !first {

	    				&self.openends.push(self.last);

	    			}

	    			let ten_millis = time::Duration::from_millis(1000);
					thread::sleep(ten_millis);

	   				pickedmove = true;

   				}


    		} else if command.trim() == "-q" {

    			quit = true;

    		}

    		&self.graphics(7);

   		}

   		quit

    }

    pub fn gameloop(&mut self) {

    	let mut curentplayer = 1;
    	let mut quit: bool = false;

   		while self.player1.pile.len() > 0 &&
    		  self.player2.pile.len() > 0 &&
    		  !quit {

    		if curentplayer == 1 {

    			quit = self.pick_loop(1);
    			curentplayer = 2;

    		}
    		else{

    			quit = self.pick_loop(2);
    			curentplayer = 1;

    		}

    	}

   		if self.player1.pile.len() == 0 {

    		println!("dominogame: ~ $ {} IS THE WINNER", &self.player1.name);

    	}else if self.player2.pile.len() == 0 {

    		println!("dominogame: ~ $ {} IS THE WINNER", &self.player2.name);

    	}

    	println!();
    	println!("dominogame: ~ $ GOOD GAME, BYE!");
    	self.graphics(7);

    }
}

fn main(){

    let mut again: bool = true;

    while again {

		let mut gameplay = Game::new();

		gameplay.gameloop();

		let mut command = String::new();

	    println!("dominogame: ~ $ Would you like to play again?");
	    println!();
    	println!("dominogame: ~ $ ([-y] for Yes and [-n] for No): "); 

	    match io::stdin().read_line(&mut command) {
        	Err(e) => panic!("couldn't read the command: {}", e),
   			Ok(f) => f,
   		};

   		if command.trim() == "-n" {

   			again = false;

   		} else if command.trim() == "-y" {  	
   		}else{

   			again = false;
   			
   		}

    }

}