```code
[= G <($x $y) [~ [xx $x] [yy [%%] $y]
                [* 2 3]
                [stdout [%%]]
                [return 0x145FD]
            ]
    |   fn (dec , dec) hex>
];

[= F <($x $y) [~ [y 23 x] [stdout [%%]]
                        [return 1]
            ] | fn (dec , fn (dec, dec) hex) dec>
];

[F 24 G];

[= buf <{1, 2, 3, 3, 5, 5}| array 5 dec>];

[= d <243.34|dec>];
[= h 0x234A34];
[= o <0o3466|oct>];
[= b <0b101001|bin>];

[= dd <{{1, 2, 3, 4, 5, 6, 7, 8}
       {3, 4, 5, 6, 7, 8, 8, 9}}|matrix 2x8 dec>
];

[= a <true|bool>];
[= b <false|bool>];
[= c <{'s', 't', 'r', 'i', 'n', 'g', '\0'} | array 7 char>];
[= ch 'B'];

[~ [GET c 1] [stdout [%%]]];
[SET c 4 'y'];
[SET dd 1x2 4545];
[pop dd 2];

[PUSH dd -1 <300|dec>];

[SET XX [+ 4 [rows dd]] 434];

[G [GET dd 1] [GET dd 3]];

[~ [&& [< [get dd 2] 23]  [== 43 43]]
    [~ [G 23] [continue]]
];

```
