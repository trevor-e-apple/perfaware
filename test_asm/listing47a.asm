; ========================================================================
; LISTING 48
; ========================================================================

bits 16

;; Immediates
mov bx, -10
add bx, -10 ; add two negative numbers
add bx, 19 ; add positive to negative, negative result
add bx, 2 ; add positive to negative, positive result
add bx, -3 ; add negative to positive, negative result
mov bx, 10
add bx, -9 ; add negative to positive, positive result

mov bx, -1
sub bx, 1 ; sub positive from negative, negative result
mov bx, 1
sub bx, 2 ; sub positive from positive, negative result
mov bx, 1
sub bx, -1 ; sub negative from positive, positive result
mov bx, -1
sub bx, -2 ; sub negative from negative, positive result

mov bx, 1
cmp bx, -1 ; cmp positive to negative
mov bx, -1
cmp bx, 1 ; cmp negative to positive

;; registers
; sub two negative numbers, negative result
mov bx, -1
mov cx, -2
sub cx, bx

; sub two negative numbers, positive result
mov bx, -1
mov cx, -2
sub bx, cx