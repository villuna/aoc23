Input ← &fras "../input/day11.txt"

Coords ← ⊚=@#⊜∘≠@\n.
Expand ← ⍉+⊃∘(×-1:⋅⊙∘)≡(⊃⋅∘(⊏:)\+∊⊙°□⊃(⇡⋅⋅∘)(⊙∘)□⊚¬∊⇡⊙(.:).+1/↥.)⍉
Solve ← ÷2/+♭⊠(/+⌵-).Expand Coords Input

Solve 2
Solve 1000000

# short version:
#
# s ← ÷2/+♭⊠(/+⌵-).⍉+⊃∘(×-1:⋅⊙∘)≡(⊃⋅∘(⊏:)
# \+∊⊙°□⊃(⇡⋅⋅∘)(⊙∘)□⊚¬∊⇡⊙(.:).+1/↥.)⍉⊚=@#
# ⊜∘≠@\n.&fras "../input/day11.txt"
# s 2
# s 1000000
