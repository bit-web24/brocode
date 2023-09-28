```code
[= G <(x y) [xx x] . [yy %% y]
                   . [stdout %%]
                   . [return 1]
     |fn (dec 0 1x1 , dec 2 1x1) dec 0 1x1>
];

[= F <(x y) [y 23 x].[stdout %%].[return 1] | fn (1x1 dec 0 , fn) 1x1 dec 0>];

[= buf <1 2 3 3 5 5|1x8 dec 0>];

[= d <243.34|1x1 dec 2>];
[= h <234A34|1x1 hex>];
[= o <3466|1x1 oct>];
[= bi <101001|1x1 bin>];

[= dd <<1 2 3 4 5 6 7 8>
       <3 4 5 6 7 8 8 9>|2x8 dec 0>
];

[= a <true|1x1 bool>];
[= b <false|1x1 bool>];
[= c <'s' 't' 'r' 'i' 'n' 'g'|1x6 char>];
[= ch <'B'|1x1 char>];

[get dd 1] . [stdout %%];
[set dd 3 20];

[pop dd <2|1x1 dec 0>];

[push dd <-1|1x1 dec 0>
         <300|1x1 dec 0>
];

[set dd [+ <4|1x1 dec 0> [rows dd]] <434|1x1 dec 0>];

[G [get dd <1|1x1 dec 0>] [get dd <3|1x1 dec 0>]];

[~ [&& [> [get dd <2|1x1 dec 0>] <23|1x1 dec 0>]  [== <43|1x1 dec 0> <43|1x1 dec 0>]]
    [G <23|1x1 dec 0>] . [continue]
];

[= x <0|1x1 dec 0>];

[~ [< x <10|1x1 dec 0>]
   [?: [!= x <5|dec 0 1x1>] [stdout x] . [set x [+ x <1|1x1 dec 0>]]  [break]]
];

[+ a b] . [G %% [get dd 1]]
        . [stdout %%];

[= x <10|dec 0 1x1>];
[= y <30|dec 0 1x1>];
[= z [+ x y]];

[= z <0|dec 0 1x1>];
[set z [+ x y]];

[= int <1x1 dec 0|metadata>];
[= x <32|int>];
```
