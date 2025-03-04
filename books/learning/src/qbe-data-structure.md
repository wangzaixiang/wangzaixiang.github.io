# QBE 核心数据结构

## Fn

```c
struct Fn {
	Blk *start;         // start block, Blk::link 构成了 Blk 链表
	Tmp *tmp;
	Con *con;
	Mem *mem;
	int ntmp;
	int ncon;
	int nmem;
	uint nblk;
	int retty; /* index in typ[], -1 if no aggregate return */
	Ref retr;
	Blk **rpo;      // reverse post order
	bits reg;
	int slot;
	int salign;
	char vararg;
	char dynalloc;
	char leaf;
	char name[NString];
	Lnk lnk;
};
```

## Blk

```c
struct Blk {
	Phi *phi;
	Ins *ins;
	uint nins;
	struct {
		short type;
		Ref arg;
	} jmp;
	Blk *s1;        // branch1 or jump
	Blk *s2;        // branch2
	Blk *link;      // next block in lexical order

	uint id;        // RPO index
	uint visit;

	Blk *idom;     // ???
	Blk *dom, *dlink;
	Blk **fron;
	uint nfron;

	Blk **pred;     // predecessors, pred->s1 == this || pred->s2 == this
	uint npred;
	BSet in[1], out[1], gen[1];
	int nlive[2];
	int loop;
	char name[NString];
};
```
