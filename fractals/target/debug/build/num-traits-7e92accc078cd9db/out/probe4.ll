; ModuleID = 'probe4.1c8df24b88887571-cgu.0'
source_filename = "probe4.1c8df24b88887571-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@alloc_2a4fd2b3e1246dfd934632606fb0b53b = private unnamed_addr constant <{ [83 x i8] }> <{ [83 x i8] c"/private/tmp/rust-20230923-6732-1s9ut5/rustc-1.72.1-src/library/core/src/num/mod.rs" }>, align 1
@alloc_a956cad42ee31fd5b2ebaf0d5e852b5a = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_2a4fd2b3e1246dfd934632606fb0b53b, [16 x i8] c"S\00\00\00\00\00\00\00w\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17h7ee255c1c7224d47E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h702708dbb5ce061dE.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h8a172b59cb9d6bd4E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_a956cad42ee31fd5b2ebaf0d5e852b5a) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h702708dbb5ce061dE.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h8a172b59cb9d6bd4E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
