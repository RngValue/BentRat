use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use ini::Ini;

pub fn check_and_fix_config(path: &str) {
    if Path::new(path).exists() == false {
        println!("{}", path);

        let _ = fs::create_dir_all(path);
        //UBUNTU
        let mut ascii_files = File::create(format!("{}/Ubuntu.txt", path)).expect("Couldn't create file!");
        ascii_files.write_all(b"                         =#%%#=    
              :-=+****+:-@@@@@@-   
          .:.*#########=.%@@@@%.   
        .=++- =#########+:-==-:.   
       :++++++.::.    .:=*#**###=  
      :++++++:            :*#####= 
      +++++=                +#####:
.+###+.:+++.                 #####*
*#####* =++                  ::::::
-#####- +++.                 %@@@@#
  ::: :++++=                *@@@@@=
      :++++++:            :#@@@@@* 
       :++++++- :.     :=#@@@@@@*  
        .=+++:.#@@%%%%@@@#=-==*-   
          .-.:%@@@@@@@@%::=+++-    
             .-+*#%%%%%=.++++++-   
                         =+++++.   
").expect("Couldn't write into file.");
        
        //WINDOWS
        ascii_files = File::create(format!("{}/Windows.txt", path)).expect("Couldn't create file!");
        ascii_files.write_all(b"                         .....:::::
           ...  ::::--------------:
.:::::--------  ------------------:
--------------  ------------------:
--------------  ------------------:
--------------  ------------------:
--------------  ------------------:
--------------  ------------------:
..............  ......             
::::::::::::::  ::::::::::::::::::.
--------------  ------------------:
--------------  ------------------:
--------------  ------------------:
--------------  ------------------:
--------------  ------------------:
    .....::::-  ------------------:
                   ....:::::------:
").expect("Couldn't write into file.");
        //ARCH LINUX
        ascii_files = File::create(format!("{}/Arch Linux.txt", path)).expect("Couldn't create file!");
        ascii_files.write_all(b"                 :                 
                :=-                
               :===:               
              .=====:              
             .-======.             
             .:=======.            
            -=-:--=====.           
           -============.          
          -==============.         
        .-================.        
       .-======:. .:=======.       
      .=======.     .=======:      
     .=======-       :====--=:     
    :========:       .=====-::     
   :======--:.       .:--=====-.   
  :===-:.                 .:--==-  
 --:.                         .:-- 
").expect("Couldn't write into file.");
        //LINUX MINT
        ascii_files = File::create(format!("{}/Linux Mint.txt", path)).expect("Couldn't create file!");
        ascii_files.write_all(b"                                   
              ..:::...             
         .:-===========-:.         
       :-=================-:       
     .-=:.:=================-.     
    .===   ===-.   .:    :====:    
   .====   ==-  .:.   ::  .====.   
   -====   ==:  -==  .==.  ====-   
   -====   ==:  -==  .==.  =====   
   -====   ==:  -==  .==.  =====   
   :====   ==-::-==:::==.  ====:   
    -===:  .-----------.  :====    
     -===-.             .:===-     
      :=====-----------=====:      
        :-===============-:        
           .:--=====--:.           
                                   
").expect("Couldn't write into file.");
        //ENDEAVOUROS
        ascii_files = File::create(format!("{}/EndeavourOS.txt", path)).expect("Couldn't create file!");
        ascii_files.write_all(b"                                   
                                   
                  .=+:             
                 -++++=:           
               :=+++++++-.         
             .-++++++++++=:        
            -=+++++++++++++-.      
          :==+++++++++++++++-:     
        .===+++++++++++++++++--.   
       -==+++++++++++++++++++=--.  
     :===+++++++++++++++++++++---. 
   .-===++++++++++++++++++++++=--- 
  -====+++++++++++++++++++++++=--- 
 ..::=+++++++++++++++++++++++=---: 
    :--------------=======-----:.  
   .::::::::::::::::.......        
                                   
").expect("Couldn't write into file.");
        //DEFAULT
        ascii_files = File::create(format!("{}/Default.txt", path)).expect("Couldn't create file!");
        ascii_files.write_all(b"                                   
               :-==-:              
              *%%%%%%*             
             -#*%#=+%%:            
             --+*=#:#%=            
             -*=====%%=            
             #=:--:.-%%-           
           :##       +%%+          
          =##.        *###:        
         +##.          #+%%.       
        .%+=           -=#%=       
        =**=.         =%%%*=       
     .-====+%*.      -==+===:      
     :=======#:      ========:     
     -========++=-=+#======-:      
      .:::-===-:. ..:====-         
                                   
").expect("Couldn't write into file.");

        let mut conf = Ini::new();
        conf.with_section(Some("stats"))
            .set("start_line", "0")
            .set("show_system_name", "true")
            .set("show_cpu_usage", "true")
            .set("show_ram_capacity", "true")
            .set("show_ram_usage", "true");
        conf.with_section(Some("misc"))
            .set("use_custom_ascii_art", "false")
            .set("custom_ascii_art", "Default")
            .set("exit_text", "press [CTRL + C] to exit...");
        conf.with_section(Some("system_color"))
            .set("use_custom_color", "false")
            .set("r", "255")
            .set("g", "255")
            .set("b", "255");
        conf.with_section(Some("accent_color"))
            .set("r", "0")
            .set("g", "110")
            .set("b", "255");
        conf.with_section(Some("good_color"))
            .set("r", "50")
            .set("g", "200")
            .set("b", "0");
        conf.with_section(Some("ok_color"))
            .set("r", "255")
            .set("g", "255")
            .set("b", "0");
        conf.with_section(Some("bad_color"))
            .set("r", "190")
            .set("g", "0")
            .set("b", "0");
        conf.write_to_file(format!("{}/Config.ini", path)).unwrap();
    }
}