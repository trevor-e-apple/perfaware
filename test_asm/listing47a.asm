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

; add large negative to large positive
; add large positive to large negative

;; registers
; add two negative numbers
; add positive to negative, negative result
; add positive to negative, positive result
; add negative to positive, negative result
; add negative to positive, positive result

; sub positive to negative, negative result
; sub positive to negative, positive result
; sub negative to positive, negative result
; sub negative to positive, positive result

; cmp positive to negative, negative result
; cmp positive to negative, positive result
; cmp negative to positive, negative result
; cmp negative to positive, positive result
