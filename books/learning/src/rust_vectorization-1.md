本篇分析一下 rust 语言的编译期自动向量化的特性。

```rust
#[inline(never)]
pub fn select(v1: &[i32], v2: &[i32], result: &mut [bool]) {
    assert!(v1.len() == v2.len() && v1.len() == result.len());
    for i in 0..v1.len() {
        if v1[i] > v2[i] {
            result[i] = true
        } else {
            result[i] = false
        }
    }
}
```

1. %rdi : %rsi  v1.ptr() : v1.len()
2. %rdx : %rcx  v2.ptr() : v2.len()
3. %r8  : %r9   result.ptr() : result.len()

```asm
LCPI6_0:
	.quad	72340172838076673   -- 0x01010101_01010101  // 8 个 1
	.section	__TEXT,__text,regular,pure_instructions
	.p2align	4, 0x90
__ZN5l_asm5demo16select17h43db37ec056aed21E:
	.cfi_startproc
	cmp	rsi, rcx    -- v1.len() == v2.len()
	jne	LBB6_10
	cmp	rsi, r9     -- v1.len() == result.len()
	jne	LBB6_10
	test	rsi, rsi  -- v1.len() == 0
	je	LBB6_9
	cmp	rsi, 32     -- v1.len() >= 32
	jae	LBB6_5
	xor	eax, eax
	jmp	LBB6_8     -- 少于32时，直接循环处理 
LBB6_5:
	mov	rax, rsi
	and	rax, -32    -- rax = rsi & -32
	xor	ecx, ecx
	vpbroadcastq	ymm0, qword ptr [rip + LCPI6_0]  -- ymm0: 32x8
	.p2align	4, 0x90
LBB6_6:
	vmovdqu	ymm1, ymmword ptr [rdi + 4*rcx]       -- ymm1..ymm4 加载 32 个 i32 from v1
	vmovdqu	ymm2, ymmword ptr [rdi + 4*rcx + 32]
	vmovdqu	ymm3, ymmword ptr [rdi + 4*rcx + 64]
	vmovdqu	ymm4, ymmword ptr [rdi + 4*rcx + 96]
	
	vpcmpgtd	ymm1, ymm1, ymmword ptr [rdx + 4*rcx]
	-- ymm1:8x32 = [ c0, c1, ..., c7 ]
	 
	vpcmpgtd	ymm2, ymm2, ymmword ptr [rdx + 4*rcx + 32] 
	-- ymm2:8x32 = [ c8, c9, ..., c15]
	 
	vpcmpgtd	ymm3, ymm3, ymmword ptr [rdx + 4*rcx + 64]  
	-- ymm3: 8x32 = [ c16, c17, ..., c23]
	
	vpackssdw	ymm1, ymm1, ymm2  
	-- ymm1: 16x16 = [ c0, c1, ..., c15] 
	
	vpcmpgtd	ymm2, ymm4, ymmword ptr [rdx + 4*rcx + 96]
	vpackssdw	ymm2, ymm3, ymm2  -- ym2 = 16..31, 【i16, 16]
	-- ymm2: 16x16 = [c16, c17, ..., c31]
	
	vpermq	ymm2, ymm2, 216 -- 0b11_01_10_00
	-- ymm2: 16x16 = [ c16, c17, c18, c19, c24, c25, c26, c27, c20, c21, c22, c23, c28, c29, c30, c31]
	-- vpermq 分析有些问题
	vpermq	ymm1, ymm1, 216
	-- ymm1: 16x16 = [ c0, c1, c2, c3,    c8, c9, c10, c11,   c4, c5, c6, c7,   c12, c13, c14, c15]
	
	vpacksswb	ymm1, ymm1, ymm2
	-- ymm1: 32x8 = [ c0, c1, c2, c3 ,  c8, c9, c10, c11,   c4, c5, c6, c7,   c12, c13, c14, c15,
	            c16, c17, c18, c19, c24, c25, c26, c27, c20, c21, c22, c23, c28, c29, c30, c31] -- 32x8
	vpermq	ymm1, ymm1, 216
	-- ymm1: 32x8 = [ c0, c1, c2, c3,   c8, c9, c10, c11,  c16, c17, c18, c19,  c24, c25, c26, c27,
	  .......]
	  
	vpand	ymm1, ymm1, ymm0
	
	vmovdqu	ymmword ptr [r8 + rcx], ymm1
	add	rcx, 32      --  一次循环处理完32个整数
	cmp	rax, rcx
	jne	LBB6_6
	cmp	rax, rsi
	je	LBB6_9
	.p2align	4, 0x90
LBB6_8:
	mov	ecx, dword ptr [rdi + 4*rax]
	cmp	ecx, dword ptr [rdx + 4*rax]
	setg	byte ptr [r8 + rax]
	lea	rcx, [rax + 1]
	mov	rax, rcx
	cmp	rsi, rcx
	jne	LBB6_8
LBB6_9:
	vzeroupper
	ret
LBB6_10:
	push	rbp
	.cfi_def_cfa_offset 16
	.cfi_offset rbp, -16
	mov	rbp, rsp
	.cfi_def_cfa_register rbp
	lea	rdi, [rip + l___unnamed_3]
	lea	rdx, [rip + l___unnamed_4]
	mov	esi, 66
	call	__ZN4core9panicking5panic17h2a3e12572053020cE
	.cfi_endproc
```

从这段代码来看，在 +avx2 特性下，编译期生成了使用 256 bit 寄存器的代码，一次循环可以处理 32 个 i32 数据。
而如果在 +avx512f 特性下，编译期生成了使用 512bit 的代码， 一次循环可以处理 64 个 i32 数据。
实际性能如何？需要找一台支持 AVX512 指令集的机器来做一下测试。

## 调整
1. 修改为 i32 与 i16 的比较:
    ```rust
    use std::simd::i8x1;

    #[inline(never)]
    pub fn select(v1: &[i32], v2: &[i16], result: &mut [bool]) {
        assert!(v1.len() == v2.len() && v1.len() == result.len());
        for i in 0..v1.len() {
            if v1[i] > (v2[i] as i32) {
                result[i] = true
            } else {
                result[i] = false
            }
        }
    }
    ```
    在 +avx2 特性下，可以使用 vpmovsxwd 指令在读取 v2 的数据时，一次将 8 个 i16 读取并转换为 8 个 i32，然后再进行比较。
    ```asm
2. 修改 `v1[i]` 为一个 `v1.get(i)` 时，生成代码？  
   此时，load 数据这一块可能会无法使用 SIMD 指令集，可能需要多次获取数据后，再拼装为一个 SIMD 寄存器。
3. 在 OLAP 向量计算中，如果采用代码生成的方式，相比解释表达式，并分派给多个模版方法，肯定会有性能上的提升：
   - 多个运算间，可以复用寄存器
   - 是否可以采用 LLVM 来做这个的代码生成？
   - 尝试阅读 LLVM IR 代码，评估后续通过生成 LLVM IR 的方式来执行的可能行。

## LLVM-IR 阅读

```
; l_asm::demo1::select
; Function Attrs: noinline uwtable
define internal fastcc void @_ZN5l_asm5demo16select17h43db37ec056aed21E(
    ptr noalias nocapture noundef nonnull readonly align 4 %v1.0, i64 noundef %v1.1,
    ptr noalias nocapture noundef nonnull readonly align 4 %v2.0, i64 noundef %v2.1,
    ptr noalias nocapture noundef nonnull writeonly align 1 %result.0, i64 noundef %result.1) unnamed_addr #0 {
start:
  %_4 = icmp eq i64 %v1.1, %v2.1
  %_7 = icmp eq i64 %v1.1, %result.1
  %or.cond = and i1 %_4, %_7  -- i1: bit
  br i1 %or.cond, label %bb5.preheader.split, label %bb4  -- br type iftrue ifalse

bb5.preheader.split:                              ; preds = %start
  %_218.not = icmp eq i64 %v1.1, 0
  br i1 %_218.not, label %bb15, label %bb13.preheader

bb13.preheader:                                   ; preds = %bb5.preheader.split
  %min.iters.check = icmp ult i64 %v1.1, 32       -- unsigned less than
  br i1 %min.iters.check, label %bb13.preheader17, label %vector.ph

vector.ph:                                        ; preds = %bb13.preheader
  %n.vec = and i64 %v1.1, -32
  br label %vector.body

vector.body:                                      ; preds = %vector.body, %vector.ph
  %index = phi i64 [ 0, %vector.ph ], [ %index.next, %vector.body ]  -- TODO? what is phi
  %0 = getelementptr inbounds [0 x i32], ptr %v1.0, i64 0, i64 %index  -- %0 = %v1.0
  %1 = getelementptr inbounds i32, ptr %0, i64 8                       -- %1 = %0 + 32byte
  %2 = getelementptr inbounds i32, ptr %0, i64 16
  %3 = getelementptr inbounds i32, ptr %0, i64 24
  %wide.load = load <8 x i32>, ptr %0, align 4                          -- 8x32 from v1
  %wide.load10 = load <8 x i32>, ptr %1, align 4
  %wide.load11 = load <8 x i32>, ptr %2, align 4
  %wide.load12 = load <8 x i32>, ptr %3, align 4
  
  %4 = getelementptr inbounds [0 x i32], ptr %v2.0, i64 0, i64 %index
  %5 = getelementptr inbounds i32, ptr %4, i64 8
  %6 = getelementptr inbounds i32, ptr %4, i64 16
  %7 = getelementptr inbounds i32, ptr %4, i64 24
  %wide.load13 = load <8 x i32>, ptr %4, align 4                        -- 8x32 from v1
  %wide.load14 = load <8 x i32>, ptr %5, align 4
  %wide.load15 = load <8 x i32>, ptr %6, align 4
  %wide.load16 = load <8 x i32>, ptr %7, align 4
  
  %8 = icmp sgt <8 x i32> %wide.load, %wide.load13                   -- signed greater than
  %9 = icmp sgt <8 x i32> %wide.load10, %wide.load14
  %10 = icmp sgt <8 x i32> %wide.load11, %wide.load15
  %11 = icmp sgt <8 x i32> %wide.load12, %wide.load16
  
  %12 = zext <8 x i1> %8 to <8 x i8>                                 -- zero extend 8x1 to 8x8
  %13 = zext <8 x i1> %9 to <8 x i8>
  %14 = zext <8 x i1> %10 to <8 x i8>
  %15 = zext <8 x i1> %11 to <8 x i8>
  
  %16 = getelementptr inbounds [0 x i8], ptr %result.0, i64 0, i64 %index
  %17 = getelementptr inbounds i8, ptr %16, i64 8
  %18 = getelementptr inbounds i8, ptr %16, i64 16
  %19 = getelementptr inbounds i8, ptr %16, i64 24
  
  store <8 x i8> %12, ptr %16, align 1                               -- store 8x8 to result
  store <8 x i8> %13, ptr %17, align 1
  store <8 x i8> %14, ptr %18, align 1
  store <8 x i8> %15, ptr %19, align 1
  
  %index.next = add nuw i64 %index, 32
  %20 = icmp eq i64 %index.next, %n.vec
  br i1 %20, label %middle.block, label %vector.body, !llvm.loop !17  -- TODO what's !llvm.loop !17

middle.block:                                     ; preds = %vector.body
  %cmp.n = icmp eq i64 %n.vec, %v1.1
  br i1 %cmp.n, label %bb15, label %bb13.preheader17

bb13.preheader17:                                 ; preds = %bb13.preheader, %middle.block
  %iter.sroa.0.09.ph = phi i64 [ 0, %bb13.preheader ], [ %n.vec, %middle.block ]
  br label %bb13

bb4:                                              ; preds = %start
; call core::panicking::panic
  tail call void @_ZN4core9panicking5panic17h2a3e12572053020cE(ptr noalias noundef nonnull readonly align 1 @alloc_882a6b32f40210455571ae125dfbea95, i64 noundef 66, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @alloc_649ca88820fbe63b563e38f24e967ee7) #12
  unreachable

bb15:                                             ; preds = %bb13, %middle.block, %bb5.preheader.split
  ret void

bb13:                                             ; preds = %bb13.preheader17, %bb13
  %iter.sroa.0.09 = phi i64 [ %_0.i, %bb13 ], [ %iter.sroa.0.09.ph, %bb13.preheader17 ]
  %_0.i = add nuw i64 %iter.sroa.0.09, 1
  %21 = getelementptr inbounds [0 x i32], ptr %v1.0, i64 0, i64 %iter.sroa.0.09
  %_13 = load i32, ptr %21, align 4, !noundef !4
  %22 = getelementptr inbounds [0 x i32], ptr %v2.0, i64 0, i64 %iter.sroa.0.09
  %_15 = load i32, ptr %22, align 4, !noundef !4
  %_12 = icmp sgt i32 %_13, %_15
  %spec.select = zext i1 %_12 to i8
  %23 = getelementptr inbounds [0 x i8], ptr %result.0, i64 0, i64 %iter.sroa.0.09
  store i8 %spec.select, ptr %23, align 1
  %exitcond.not = icmp eq i64 %_0.i, %v1.1
  br i1 %exitcond.not, label %bb15, label %bb13, !llvm.loop !20
}
```

对照 [LLVM-IR 文档](https://llvm.org/docs/LangRef.html#phi-instruction)， 还是比较好理解的， 相比 x86 汇编，LLVM-IR 在 SIMD
上的可读性显然要高太多。如果理解了 LLVM-IR，并掌握了生成 LLVM-IR 后再通过 LLVM 生成机器码，然后再通过 JIT 的方式执行，那么，在 OLAP 中未尝
不是一种更好的替代模版特化的方式。
1. 对于较为复杂的表达式，例如 a > b && c + d > e, 特化的方式，基本上每个运算符都是一次函数调用，这里是4次调用，且每次函数调用涉及到类型组合，
   需要特化的函数版本会非常的多。
2. 使用 LLVM-IR，这个表达式可以直接优化为 1个函数调用，然后通过 LLVM 优化器，生成最优的机器码。内部可能会减少不必要的 Load/Store 过程，减少
   中间向量的生成和内存占用。

JIT 参考资料： 
1. [Create Your Own Programming Language with Rust](https://createlang.rs/01_calculator/basic_llvm.html)
2. [Building a JIT](https://llvm.org/docs/tutorial/BuildingAJIT1.html)