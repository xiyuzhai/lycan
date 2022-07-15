def m : Nat := 1
def n: Nat := 0
def b1: Bool := true
def b2 : Bool := false

#check m
#check n
#check n + 0        -- Nat
#check m * (n + 0)  -- Nat
#check b1           -- Bool
#check b1 && b2     -- "&&" is the Boolean and
#check b1 || b2     -- Boolean or
#check true         -- Boolean "true"


#eval 5 * 4         -- 20
#eval m + 2         -- 3
#eval b1 && b2      -- false