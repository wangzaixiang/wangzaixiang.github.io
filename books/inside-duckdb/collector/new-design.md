
# Data Type
- boolean
- integers
  - i8, i16, i32, i64
- floating point numbers
  - float, double
- strings
- date and time
- compound types
  - list
  - struct/tuple

# Vector
```
Vector<i32>

Vector<[i32]>

struct user {
    i32 id;
    string name;
}

Vector<user> 
```

# Vector Layout
1. Flat Vector
2. Dictionary Vector
   - backed by a flat vector
3. Structure Of Vector  -> Layout
   - each column is stored in a separate vector

```
struct customers1 {
   customer_id: i32,
   name: string,
   gender: string,
}

struct customers2 {
   customer_id: i32,
   name : string,
}

a : Vector<customers1> 
a.customer_id: Vector<i32>
a.name: Vector<string>



struct sale_orders1 {
   customer_id: i32,
   freight: f64,
}
struct join1 {
   left: i32,
   right: i32,
}
struct join2 {
   freight: f64,
   name: string,
}

v7: Vector<sale_orders1>

v8: Vector<join1> = join(v6, v7, v6.customer_id, v7.customer_id)
v9: Vector<join2> = make_struct(
       "freight": select(v8.left, v6.freight),
       "name": select(v8.right, v7.name)
    )
    
v10: Vector<(name: string, count: i64, sum: i64)> =
    group_by(v9, [v9.name], 
        [ count(v9.freight),
            sum(v9.freight)
        ]
    )
    
def pipeline1(){
    v1: Vector<bool> = filter_eq( a.gender,  'M' )
    v2: Vector<bool> = filter_>( a.name, 'abc' )
    v3: Vector<bool> = filter_<( a.name, 'abd' )
    v4: Vector<bool>  = v1 & v2 & v3

    v5: Vector<customers1> = filter(a, v4)
    v6: Vector<customers2> = project(v5, [customer_id, name])
    ht1: HashTable<string, i64> = build_hash_table(v6.name, v6.customer_id)
}

def pipeline2(){
}
    

```

# 调整

```
// scan( customers ) |> filter |> build_hash
def pipeline1() {
    // Vector<(customer_id:i32, name:string, gender:string)>
    v1 = table_scan :table = "customers"
            :columns = 
                "customer_id": { data_type: "i32", nullable: false, ordered: 0, unique: true },
                "name": { data_type: "string", nullable: false, ordered: 0, unique: false },
                "gender": { data_type: "string", nullable: false, ordered: 0, unique: false }
            :minmaxes = 
                "gender": ["M", "M"],
                "name": ["abc", "abd"]
    // Vector<(customer_id:i32, name:string)>            
    v2 = filter :input = v1
            :expr = $in.gender == "M" && ($in.name >= "abc" && $in.name < "abd")
            :projection = [ "customer_id", "name" ]
   
    // Tuple
    minmax = aggregate :input = v2.customer_id
            :aggregates = 
                "min": min
                "max": max
            
    ht1 = build_hash :input = v2
            :key = "customer_id"
            :value = "name"        
}

// scan( sale_orders ) |> filter | hash_join |> project |> hash_group_by
def pipeline2() {
    // Vector<(customer_id:i32, freight:f64)>
    v1 = table_scan :table = "sale_orders"
            :columns = 
                "customer_id": { data_type: "i32", nullable: false, ordered: 0, unique: true },
                "freight": { data_type: "f64", nullable: false, ordered: 0, unique: false }
            :minmaxes = 
                "freight": [10, 50]
                "customer_id": [pipeline1.minmax.min, pipeline1.minmax.max]
                
    // Vector<(customer_id:i32, freight:f64)>            
    v2 = filter :input = v1
            :expr = $in.freight > 10 && $in.freight < 50
            :projection = [ "customer_id", "freight" ]
            
    // Vector<(customer_id:i32, name:string, freight:f64)>
    v3 = hash_left_join 
            :ht = pipeline1.ht1 
            :key = v2.customer_id
            :projection = [ v2.customer_id, $ht.name, v2.freight ]
    
    // Vector<(name:string, count:i64, sum:i64)>        
    v4 = hash_group_by :input = v3
            :group_by = [ "name" ]
            :aggregates = 
                "count": count($in.freight)
                "sum": sum($in.freight)
}

```