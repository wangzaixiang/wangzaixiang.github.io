# QBE 源代码阅读 3: fillpreds, filluse

## fillpreds

pass fillpreds 是一个 psuedo pass，其职责是完善 CFG 数据结构，填充好 Blk::preds 字段。

TODO QBE 的源代码中, fillpreds 函数逻辑上是有内存泄漏的。

brk1->pred == brk2 ==> brk2.s1 == brk1 || brk2.s2 == brk1

```c
void
fillpreds(Fn *f)
{
	Blk *b;

	for (b=f->start; b; b=b->link) {    // reset predecessors, it is already setted in ???
		b->npred = 0;
		b->pred = 0;                // 这里会有内存泄漏
	}
	for (b=f->start; b; b=b->link) {
		if (b->s1)
			b->s1->npred++;
		if (b->s2 && b->s2 != b->s1)
			b->s2->npred++;
	}
	for (b=f->start; b; b=b->link) {
		if (b->s1)
			addpred(b, b->s1);
		if (b->s2 && b->s2 != b->s1)
			addpred(b, b->s2);
	}
}
```

## filluse

重点维护字段：
- Tmp::def
- Tmp::bid
- Tmp::ndef
- Tmp::nuse
- Tmp::cls
- Tmp::phi
- Tmp::width

- [ ] 对主要结构添加 to_str 方法，方便进行调试。包括： Tmp, Ins, Phi, Blk, Fn etc.
