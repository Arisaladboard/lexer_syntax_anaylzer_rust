use core::panic;
use std::env::args;
use std::fs;
use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;

fn main() {
    println!("This is a good place to start... SUCCESS TO ALL!");

    // writing program in rust that takes in a program written in square circle
    // figures out if there is any lexical or syntax errors, if so panic version error

    //gets file name and output type scheme or prolog from command line 

    let _file_name : String = args().nth(1).unwrap().to_string();    
    let _output_type : String = args().nth(2).unwrap().to_string();     
    
    let file_path = Path::new(&_file_name);
  // opens file and reads entire thing to a string 

    let mut _contents_of_file = fs::read_to_string(file_path);  

    let mut lexer_vector : Vec<Lexer> = Vec::new(); // vector that will store different strings for later token and lexeme evaluation

    let mut _lexer_lexeme_str : & mut String = & mut String::from(""); // setting up values for parser constructor 
     
    for c in _contents_of_file.unwrap().chars() { //gets the string to pass in lookup function which will evauluate the tokens
        
        if c.is_whitespace(){    continue;  } 

        if c.is_alphanumeric(){  
         _lexer_lexeme_str.push(c.clone()); //checks if it is a letter or number 
            continue;
        }
            
       

        if _lexer_lexeme_str != "" {  // if it is not white space then it uses lookup function to identify the token 
            let temp_lexer : Lexer = look_up(_lexer_lexeme_str.to_string()); 
            lexer_vector.push(temp_lexer);
            _lexer_lexeme_str.clear();
            
          } 
          
        let temp_lexer : Lexer = look_up(c.to_string()); // creates a lexer class to hold the token and specific lexeme
        lexer_vector.push(temp_lexer); // adds to vector 
        _lexer_lexeme_str.clear();

    }
    let mut temp_key  : String = String::from("");
    let mut hash : HashMap<String, Symbol> = HashMap::new();   // sets up more values for parser class   
    

    let mut new_parser = Parser{    // creates parser class to begin analyzing the lexical and syntax 
            lexical_vector : lexer_vector,
            i : 0,
            symbol_table : hash,
            hash_key : temp_key,
            output_type : _output_type.to_string(),
            

    };
    new_parser.program_start(); // begins lexical and syntax analyzer 
    
    println!("Both Lexical and Syntax tests pass" );
    
    // for (_key, value) in &new_parser.symbol_table {
    //     value.to_string();
    // }

    // if approved, then moves on to see if -s or -p will translate a program that will output the operations 
    // in scheme or prolog,,, it will just produce the output, not have it connected yet 

}

#[derive(Debug, PartialEq)]
#[derive(Copy, Clone)]
enum Token {   // ENUMS that classify the tokens used in this grammar 
    ID, 
    NUM,
    SEMICOLON,
    COLON,
    COMMA,
    PERIOD,
    LPAREN,
    RPAREN,
    ASSIGN,
    DEFINITIONS,
    OPERATIONS,
    POINT,
    CIRCLE,
    SQUARE,
    PRINT,
    CONTAINED,
    INTERSECTS,
    END,
    NONE

}


struct Lexer { // making the lecture structs that will be compared in the lexerical analyzer 

    
    lexeme : String,
    token : Token 


    }

impl Lexer { // simple return functions to make getting values easier from lexer struct 

    fn get_lexer(&self) -> String {
        
            return self.lexeme.to_string();
    }

    fn get_token(&self) -> Token{
        return self.token; 
    }

    
}

// checks the lexeme and makes sure it falls under one of the token values. it then creates a Lexer struct and return it with the 
// corresponding token and its specific lexeme. Passes in a string of the lexeme 
fn look_up( lex_str : String) -> Lexer {   // checks the lexeme and makes sure it falls under one of the enum tokens 
      
    // first checks if the lexeme falls under these tokens
       let mut temp_token:Token = match lex_str.as_str() {  
            
            ";" => Token::SEMICOLON,
            ":" => Token::COLON,
            "," => Token::COMMA,
            "." => Token::PERIOD,
            "(" => Token::LPAREN,
            ")" => Token::RPAREN,
            "=" => Token::ASSIGN,
            _  => Token::NONE
 
        };

        // then moves on and checks if it is a word
         // checks if the char is a letter and undercase, then matches if the 
         // correct word is found 
        
        
        if lex_str.chars().all(|c| matches!(c, 'a'..='z')){ 

           
           temp_token = match lex_str.as_str() {
                "definitions" => Token::DEFINITIONS, 
                "operations" => Token::OPERATIONS,
                "point" => Token::POINT,
                "circle" => Token::CIRCLE,
                "square" => Token::SQUARE,
                "print" => Token::PRINT,
                "contained" => Token::CONTAINED,
                "intersects" => Token::INTERSECTS,
                "end" => Token::END,
                _  => Token::ID

            };


        }
            // then checks if it is a number and matches 

        if lex_str.chars().all(|c: char| matches!(c,'0'..='9')) {temp_token = Token::NUM}
        
        
        
        let new_lexer = Lexer {  

            lexeme:  lex_str.clone(),
            token: temp_token

        };

        if new_lexer.get_token() == Token::NONE{   // returns error because the string passed in doesn't fall under any tokens
            panic!("Lexical Error: {}" , new_lexer.get_lexer());
        }

       return new_lexer;
    }
    

struct Parser {  // parser is made to analyze the lexemes and syntax 

    // check if what enum it is, if it an enum that matters check it 
    
    lexical_vector : Vec<Lexer> ,
    i : usize,
    symbol_table :  HashMap<String, Symbol>,
    hash_key : String,
    output_type :String,
    

} 
impl Parser { // functions to make it easier to get values 

   fn return_key(& self) -> String {
    return self.hash_key.clone(); 
   }


   // program start function runs through the entire vector, and implements a top down
   // recursive function that goes through all the lexemes and makes sure the grammars are correct
   // continues to call itself and check the grammars are right. Returns errors if incorrect, if correct
   // then it checks whether the output should be prolog or scheme and prints the correct input. 

    fn program_start (& mut self,) {
            
    

        
         if self.lexical_vector[self.i].get_token() == Token::DEFINITIONS{
            self.i+=1;
            if self.lexical_vector[self.i].get_token() != Token::COLON{
               
                panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer());
            }
           self.i+=1; 

            self.defs();
            
            if self.lexical_vector[self.i].get_token() == Token::OPERATIONS{
                self.i+=1;
                if self.lexical_vector[self.i].get_token() != Token::COLON{
                  
                    panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer());
                }
                self.i+=1;
                self.operations();

         }
         
            if self.lexical_vector[self.i].get_token() != Token::END{
                
             panic!("Syntax Error: {}",self.lexical_vector[self.i].get_lexer());
            }

        }

      
     
     
    }

    // part of the top down parser, returns an error if the grammar is incorrect 
    fn defs (& mut self) {

        
        self.def(); 

        if self.lexical_vector[self.i].get_token() == Token::SEMICOLON {
            self.i+=1;
           return self.defs();
        }
        

    }
        // part of the top down parser, returns an error if the grammar or syntax is incorrect 
    fn def (& mut self){


        let mut temp_symbol : Symbol = Symbol { _type: ("".to_string()), _lhs: ("".to_string()), _rhs: ("".to_string()) }; 


        if self.lexical_vector[self.i].get_token() == Token::ID { //a 
            self.update_symbol_table(& mut temp_symbol);
            self.i+=1; 
                if self.lexical_vector[self.i].get_token() == Token::ASSIGN {
                   self.i+=1; 
                    if self.lexical_vector[self.i].get_token() == Token::POINT || self.lexical_vector[self.i].get_token() == Token::CIRCLE || self.lexical_vector[self.i].get_token() == Token::SQUARE {
                        let mut x: bool = false;
                        self.update_symbol_table(&mut temp_symbol);

                        if self.lexical_vector[self.i].get_token() == Token::POINT{
                            x = true;
                            self.i+=1; 
                        } else { self.i+=1};
                        if self.lexical_vector[self.i].get_token() == Token::LPAREN{
                            self.update_symbol_table(&mut temp_symbol);
                            self.i+=1;
                            if x {
                                if self.lexical_vector[self.i].get_token() == Token::NUM  {  // need to also have something that checks if it is a circle or num it can be an ID or num
                                    self.i+=1;
                                }
                                else{
                                    panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer());
                                }
                            } 
                            else {
                                if self.lexical_vector[self.i].get_token() == Token::ID  {  // need to also have something that checks if it is a circle or num it can be an ID or num
                                    self.i+=1;
                                }
                                else{
                                    panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer())
                                }
                            }
                        
                                    if self.lexical_vector[self.i].get_token() == Token::COMMA {
                                         self.update_symbol_table(&mut temp_symbol);
                                        self.i+=1;
                                        if self.lexical_vector[self.i].get_token() == Token::NUM {
                                            self.i+=1;
                                            if self.lexical_vector[self.i].get_token() == Token::RPAREN{
                                               
                                                self.symbol_table.insert(self.return_key(), temp_symbol); 
                                                self.i+=1;
                                                
                                            } else {panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer())}

                                        } else {panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer())}

                                    } else { panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer())}

                        } else  {panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer())}

                    } else {panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer())}
                        
                
                } else {panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer())}


        } else {panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer())}
        
     }
         // part of the top down parser, returns an error if the grammar  or syntax is incorrect 

    fn operations(& mut self){
        self.operation();

        if self.lexical_vector[self.i].get_token() == Token::SEMICOLON{
            self.i+=1;
            return self.operations();
        }

     }

         // part of the top down parser, returns an error if the grammar or syntax is incorrect 
    fn operation (& mut self){

        if self.lexical_vector[self.i].get_token() != Token::PRINT &&  self.lexical_vector[self.i].get_token() != Token::CONTAINED && self.lexical_vector[self.i].get_token() != Token::INTERSECTS{
            panic!("Syntax Error: {}", self.lexical_vector[self.i].get_lexer());
        }
        let mut print_str : String; 

        if self.lexical_vector[self.i].get_token() == Token::PRINT { 
            self.i+=1;
            if self.lexical_vector[self.i].get_token() == Token::LPAREN{
                self.i+=1;
                if self.lexical_vector[self.i].get_token() == Token::ID{

                    self.hash_key =  self.lexical_vector[self.i].get_lexer(); 
                    let temp_type : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._type.as_str());
                    let mut temp_lhs : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._lhs.as_str());
                    let temp_rhs : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._rhs.as_str());


                    if self.output_type == "-s".to_string(){ // checks if scheme or prolog output then makes the correct print stmt 
                            
                       if self.symbol_table.contains_key(&temp_lhs){
                        self.hash_key = temp_lhs.to_string();
                         temp_lhs = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._lhs.as_str());
                         let temp_lhs_rhs = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._rhs.as_str());

                         println!("(print-{} (makepoint {} {}) {})", temp_type, temp_lhs,temp_lhs_rhs, temp_rhs);
                       } else {

                         println!("(print-{} ({}) {})", temp_type, temp_lhs, temp_rhs);
                       }

                       
                        
                       
                    }else {

                            // for prolog output

                            if self.symbol_table.contains_key(&temp_lhs){
                                self.hash_key = temp_lhs;
                                 temp_lhs = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._lhs.as_str());
                                 let temp_lhs_rhs = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._rhs.as_str());
        
                                 println!("query({}(point2d({},{}), {}))", temp_type, temp_lhs,temp_lhs_rhs, temp_rhs);
                               } else {
        
                                 println!("(query({}({}), {}))", temp_type, temp_lhs, temp_rhs);
                               }

                         

                        }

                   self.i+=1;
                    if self.lexical_vector[self.i].get_token() == Token::RPAREN{
                       self.i+=1;
                        
                       
                    }
                } else {panic!("Syntax Error: {}",self.lexical_vector[self.i].get_lexer() )}
            } else {panic!("Syntax Error: {}",self.lexical_vector[self.i].get_lexer() )}
        }

        if self.lexical_vector[self.i].get_token() == Token::CONTAINED || self.lexical_vector[self.i].get_token() == Token::INTERSECTS {
            
            let mut which_operation : String = "intersects".to_string();
            if self.lexical_vector[self.i].get_token() == Token::CONTAINED{
                   which_operation = "contained".to_string();
            }
            self.i+=1;
            if self.lexical_vector[self.i].get_token() == Token::LPAREN {
                    self.i+=1; 
                    //gets first ID and stores values
                    if self.lexical_vector[self.i].get_token() == Token::ID{

                        self.hash_key =  self.lexical_vector[self.i].get_lexer(); 
                    let temp_type1 : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._type.as_str());
                    let mut temp_lhs1 : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._lhs.as_str());
                    let temp_rhs1 : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._rhs.as_str());
                     self.hash_key = temp_lhs1.to_string();
                        let temp_lhs_lhs1 = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._lhs.as_str());
                         let temp_lhs_rhs1 = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._rhs.as_str());

                    

                        
                        self.i+=1;
                       if self.lexical_vector[self.i].get_token() == Token::COMMA {
                            self.i+=1;
                            if self.lexical_vector[self.i].get_token() == Token::ID {
                                self.hash_key =  self.lexical_vector[self.i].get_lexer(); 
                                let  temp_type2 : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._type.as_str());
                                let  temp_lhs2 : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._lhs.as_str());
                                let temp_rhs2 : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._rhs.as_str());
                                 
                                self.hash_key = temp_lhs2.to_string(); 
                                let  temp_lhs_lhs2 : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._lhs.as_str()); 
                                let temp_lhs_rhs2 : String = String::from(self.symbol_table.get_key_value(&self.hash_key).unwrap().1._rhs.as_str());
            
                                
                                
                                 if which_operation == "intersects".to_string() { // checks whether to print contained or intersects
                                    if self.output_type.contains("s"){
                                        println!("(intersects-{}-{} (makepoint {} {}) {} (makepoint {} {}) {})", temp_type1, temp_type2, temp_lhs_lhs1, temp_lhs_rhs1, temp_rhs1, temp_lhs_lhs2, temp_lhs_rhs2, temp_rhs2);
                                
                                    } else {
                                        println!("query(intersects({}(point2d({},{}), {}), {}(point2d({},{}) {})))", temp_type1, temp_lhs_lhs1, temp_lhs_rhs1, temp_rhs1, temp_type2, temp_lhs_lhs2, temp_lhs_rhs2, temp_rhs2);
                                    }
                                    
                                  }else {
                                    
                                    if self.output_type.contains("s"){
                                        println!("(contained-{}-{} (makepoint {} {}) {} (makepoint {} {}) {})", temp_type1, temp_type2, temp_lhs_lhs1, temp_lhs_rhs1, temp_rhs1, temp_lhs_lhs2, temp_lhs_rhs2, temp_rhs2);
                                
                                    } else {
                                        println!("query(contained({}(point2d({},{}), {}), {}(point2d({},{}) {})))", temp_type1, temp_lhs_lhs1, temp_lhs_rhs1, temp_rhs1, temp_type2, temp_lhs_lhs2, temp_lhs_rhs2, temp_rhs2);
                                    }
                                    
                                  }
                                






                                self.i+=1;
                                if self.lexical_vector[self.i].get_token() == Token::RPAREN{
                                    self.i+=1;
                                }else {panic!("Syntax Error: {}",self.lexical_vector[self.i].get_lexer() )}
                            }else {panic!("Syntax Error: {}",self.lexical_vector[self.i].get_lexer() )}
                        } else {panic!("Syntax Error: {}",self.lexical_vector[self.i].get_lexer() )}

                    }else {panic!("Syntax Error: {}",self.lexical_vector[self.i].get_lexer() )}
                    
            } else {panic!("Syntax Error: {}",self.lexical_vector[self.i].get_lexer() )}

        } 

     }
   
    
     
    

// used in the parser to create a symbol table to later be used to help with outputting the correct output
fn update_symbol_table(& mut self, temp_sym : &mut Symbol ){

    if self.lexical_vector[self.i].get_token() == Token::ID{
       self.hash_key = self.lexical_vector[self.i].get_lexer();

    }
       

    if self.lexical_vector[self.i].get_token() == Token::POINT{
        temp_sym._type = "Point".to_string();
    }

    if self.lexical_vector[self.i].get_token() == Token::CIRCLE{
        temp_sym._type = "Circle".to_string(); 
    }
    if self.lexical_vector[self.i].get_token() == Token::SQUARE{
        temp_sym._type = "Square".to_string();
    } 

    if self.lexical_vector[self.i].get_token() == Token::LPAREN{

        temp_sym._lhs = self.lexical_vector[self.i+1].get_lexer();

    }

    if self.lexical_vector[self.i].get_token() == Token::COMMA{

        temp_sym._rhs = self.lexical_vector[self.i+1].get_lexer();
    }
    
    

    
}




} // implements parser struct that holds the token, and the left hand value and right hand value

struct Symbol {
    _type : String, 
    _lhs : String,
    _rhs : String



}

impl Symbol { // function to make it easier to access data memembers 
    fn to_string (&self){
        println!("Type: {}, Lhs: {}, Rhs {}", self._type, self._lhs, self._rhs);

    }
}