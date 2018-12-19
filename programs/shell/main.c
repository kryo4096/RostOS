#include "syscall.h"
#include "std.h"
#include "keyboard.h"
#include "vga.h"
#include "terminal.h"
#include "process.h"

void _start() {
    vt_init(); // initialize a new virtual terminal
    char exec_path[100] = "/bin/"; // buffer to hold the executable's name

    vt_println("Welcome to RostOS!");

    // loop forever
    while(1) {
        vt_print("> "); // print a prompt
        char* line = vt_readln(); // read user input (blocks until enter is pressed)
        vt_newln(); // print a new line
        
        // extend exec_path by the user input to get a null-terminated executable path
        for(int i = 0; line[i]; i++) {
            exec_path[5 + i] = line[i]; 
            exec_path[5 + i + 1] = 0;
        }

        // try executing the file located at the path
        pid_t pid = proc_spawn((path_t) &exec_path);

        // proc_spawn will return a non-zero pid if it succeeded...
        if (pid) {
            proc_wait(pid);
        } 
        // ... or 0 if it failed (we assume it's because of a missing executable)
        else {
            vt_print("Command not found: ");  // print the error message...
            vt_println(&exec_path); // ... and the invalid path
        }
    }
}
