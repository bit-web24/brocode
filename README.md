```code
[= G <(x y) [flow [xx x] [yy %% y]
                   		 [stdout %%]
                    	 [return 1]
			]
     |fn (matrix 1x1 dec , matrix 1x1 dec) matrix 1x1 dec>
];

[= F <(x y) [flow [y 23 x] [stdout %%].[return 1]] | fn (1x1 dec 0 , fn) 1x1 dec 0>];

[= buf <{1, 2, 3, 3, 5, 5}| matrix 1x8 dec>];

[= d <243.34|matrix 1x1 dec>];
[= h <234A34|matrix 1x1 hex>];
[= o <3466|matrix 1x1 oct>];
[= bi <101001|matrix 1x1 bin>];

[= dd <{{1, 2, 3, 4, 5, 6, 7, 8}
       {3, 4, 5, 6, 7, 8, 8, 9}}|matrix 2x8 dec>
];

[= a <true|matrix 1x1 bool>];
[= b <false|matrix 1x1 bool>];
[= c <{'s', 't', 'r', 'i', 'n', 'g'} | matrix 1x6 char>];
[= ch <'B'|1x1 char>];

[get dd 1] . [stdout %%];
[set dd 3 20];

[pop dd <2| matrix 1x1 dec 0>];

[push dd <-1|matrix 1x1 dec>
         <300|matrix 1x1 dec>
];

[set dd [+ <4|matrix 1x1 dec> [rows dd]] <434|matrix 1x1 dec>];

[G [get dd <1|matrix 1x1 dec>] [get dd <3|matrix 1x1 dec>]];

[~ [&& [lt [get dd <2|matrix 1x1 dec>] <23|matrix 1x1 dec>]  [== <43|matrix 1x1 dec> <43|matrix 1x1 dec>]]
    [flow [G <23|matrix 1x1 dec>] [continue]]
];

[= x <0|matrix 1x1 dec>];

[loop [< x <10|matrix 1x1 dec>]
   [ifelse [!= x <5|matrix 1x1 dec>] [flow [stdout x] [set x [+ x <1|matrix 1x1 dec>]]]  [break]]
];

[flow [+ a b] [G %% [get dd 1]] [stdout %%]];

[= x <10|matrix 1x1 dec>];
[= y <30|matrix 1x1 dec>];
[= z [+ x y]];

[= z <0|matrix 1x1 dec>];
[set z [+ x y]];

[= int <matrix 1x1 dec|metadata>];
[= x <32|int>];
```
