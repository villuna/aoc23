Input ← &fras "../input/day18.txt"

PHex ← /+×ⁿ:16⇌⇡⧻.⊗:"0123456789abcdef"
Split ← ⊜□≠⊙.
POne ← ≡(⊟⊃(⊢⊗:"RDLU"°□)(⋅⋕)°⊟↙2Split@ °□)Split@\n
PTwo ← ≡(⊟⊃(⋕↘)(PHex↙)-1⧻.⊏+2⇡-3⧻.°□⊢↘2Split@ °□)Split@\n
Dirs ← [1_0 0_1 ¯1_0 0_¯1]
Solve! ← +1÷2;∧(⊙(+×°⊟+×¯1_1).+,×⊡:Dirs°⊟):0_0⊃∘(/+≡⊡1)^1Input

Solve!POne
Solve!PTwo
