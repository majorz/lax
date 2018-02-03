# Syntax Directions


### Function

```
fn add(x y)
   x + y
```

### Call

```
z = add(x y)
o = add(20 40)
```

### Nesting

```
xs = add(mul(10 30) mul(20 30))
```

### List [1]

```
xs = 10 40 60
```

### List [2]

```
xs = dda(10) lum(5) vid(9)
```

### List [3]

```
xs = (
   dda(10)
   lum(5)
   vid(9)
)
```

### List [4]

```
xs = (20 + 40) (30 - 15)
```

### Map [1]

```
m = map
   2: 100
   6: 200
   9: 250
```

### Map [2]

```
m = map
   ^first:
      q = x * y
      p = x + y
      q + p
   ^second: 200
   ^third: 250
```

### If [1]

```
if x == 10
   10
el
   40
```

### If [2]

```
if x == 10
      and y == 20
      and z == 50
   10
el
   40
```

### Match [1]

```
y = match x
   10: 10
   _: 40
```

### Match [2]

```
match x
   10:
      y = 10
      z = 50
   _:
      y = 40
      z = 90
```
