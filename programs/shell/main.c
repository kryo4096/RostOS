#include "syscall.h"
#include "std.h"
#include "keyboard.h"
#include "terminal.h"
#include "process.h"
#include "vga.h"

bool strcmp(char * a, char * b) {
    
    for(int i = 0; a[i]; i++) {
        if(a[i]!=b[i]) {
            return false;
        }
    }

    return true;
}

void _start() {
    vt_init(); // initialize a new virtual terminal
    char exec_path[100] = "/bin/"; // buffer to hold the executable's name

    vt_println("Welcome to RostOS!");

    // loop forever
    while(1) {
        vt_print("> "); // print a prompt
        char* line = vt_readln(); // read user input (blocks until enter is pressed)

        vt_newln(); // print a new line
        
        if(!line) {
            continue;
        }

        if(strcmp(line,"matrix")) {
            vt_set_color(vga_color_code(C_BLACK, C_LIGHT_GREEN));
            continue;
        }

        if(strcmp(line,"exit")) {

            proc_exit();

        }

        bool wait = true;

        // extend exec_path by the user input to get a null-terminated executable path
        for(int i = 0; line[i]; i++) {
            
            if(line[i] == '&') {
                wait = false;
                break;
            }  
            exec_path[5 + i] = line[i]; 
            exec_path[5 + i + 1] = 0;
        }

        // try executing the file located at the path
        pid_t pid = proc_spawn((path_t) &exec_path);

        // proc_spawn will return a non-zero pid if it succeeded...
        if (pid) {
            if(wait) {
                proc_wait(pid);
            }
        } 
        // ... or 0 if it failed (we assume it's because of a missing executable)
        else {
            vt_print("Command not found: ");  // print the error message...
            vt_println(&exec_path); // ... and the invalid path
        }
    }
}
