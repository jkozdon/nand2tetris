// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// multiply by doubling R0 and adding into R2 whenever the multiple is needed
// using and to compare with entries of R1

    // val = R0
    // R2 = 0
    // i = 1
    // LOOP:
    //   if R1 & i
    //     R2 = R2 + val
    //   endif
    //   val = val + val
    //   i = i + i
    //   if i < 32768
    //      goto LOOP
    //   endif
    // END:
    // goto END

// loop forever
(END)
     @END
     M;JMP


    // val = R0
    @R0
    D = M
    @val
    M = D

    // R2 = 0
    @R2
    M = 0

    //i = 1
    @i
    M = 1

    //while i < 32768
(LOOP)
      // if R1 & i
      @i
      D = M
      @R1
      D = D&M
      @NOMUL
      D;JEQ

        //R2 = R2 + val
        @val
        D = M
        @R2
        M = M + D
(NOMUL)

      // val = val + val
      @val
      D = M
      M = M + D

      // i = i + i
      @i
      D = M
      M = M + D

      // if i < 32768 -> loop
      D = M
      @LOOP
      D;JGT

// loop forever
(END)
     @END
     M;JMP

