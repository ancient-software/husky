define i8 @main(i8 %argc, i8** %arcv) {
  %1 = mul nuw i8 -63, 2
  ret i8 %1
}

; ASSERT POISON: call i1 @main(i8 1, i8** null)
