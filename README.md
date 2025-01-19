Intel 8086 reassembly and simulation program

The tool was a project to get accustomed to a simple CPU instruction set, based on the
basic homeworks from the Part 1 of the "Performance Aware Programming Series" by Casey Muratori.

## Dependencies
The nasm assembler must be available on your system for this tool to work.

## Reassembly
The first of the functions of this tool is reassembly. In this path, the program will call nasm
to construct the machine code. Then, the program will read the machine code, reconstruct equivalent
assembly code, and construct the machine code with the new assembly. Then the two programs will be compared.
If the programs do not match, the reassembler will print out any files which had a difference between the nasm
assembled program and the one that this program assembled.

The program will accept both a file or a directorty as an argument for reassembly. If a directory is passed,
all files matching `*.asm` will be reassembled. If it is a file, only the file will be reassembled.

## Simulation
Since I didn't want to spend a long time implementing simulations for all instructions, the simulator is
a bit light on features. For example, the high and low bytes of some registers are not addressable. Still, 
several of the listings in the test_asm directory will simulate successfully. In general, loads and stores
will work, as well as a basic jump, and most of the memory and immediate arithmetic instructions.

The simulator will print out any registers that changed along with the associated instruction that ran. It will
also show the state of all registers at the end of the simulated program. 
