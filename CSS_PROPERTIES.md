#,CSS Property,Tailwind Class (closest)
1,align-content,"content-start, content-center, content-end, content-between, content-around, content-evenly"
2,align-items,"items-start, items-center, items-end, items-stretch, items-baseline"
3,align-self,"self-auto, self-start, self-center, self-end, self-stretch"
4,all,"all: unset → no direct, use reset custom"
5,animation,animate-* (built-in animations)
6,animation-delay,"delay-0, delay-100, delay-1000 etc"
7,animation-direction,inside custom keyframes only
8,animation-duration,"duration-0, duration-300, duration-1000"
9,animation-fill-mode,inside custom keyframes only
10,animation-iteration-count,"animate-spin → infinite, or custom"
11,animation-name,animate-* or custom
12,animation-play-state,animate-paused (via hover:animate-none)
13,animation-timing-function,"ease-linear, ease-in, ease-out, ease-in-out"
14,appearance,appearance-none
15,aspect-ratio,"aspect-auto, aspect-square, aspect-video"
16,backdrop-filter,"backdrop-blur, backdrop-brightness, backdrop-saturate etc"
17,backface-visibility,no direct (rarely needed)
18,background,bg-* shorthand
19,background-attachment,"bg-fixed, bg-local, bg-scroll"
20,background-blend-mode,"bg-blend-multiply, bg-blend-screen etc"
21,background-clip,"bg-clip-border, bg-clip-padding, bg-clip-content, bg-clip-text"
22,background-color,"bg-transparent, bg-current, bg-black, bg-red-500 etc"
23,background-image,"bg-gradient-to-r, bg-[url(...)]"
24,background-origin,"bg-origin-border, bg-origin-padding, bg-origin-content"
25,background-position,"bg-left, bg-center, bg-top, bg-bottom-right etc"
26,background-position-x,"bg-left, bg-right, bg-center"
27,background-position-y,"bg-top, bg-bottom, bg-center"
28,background-repeat,"bg-repeat, bg-no-repeat, bg-repeat-x, bg-repeat-y"
29,background-size,"bg-auto, bg-cover, bg-contain"
30,block-size,h-* (height)
31,border,"border, border-2, border-t-4 etc"
32,border-block,logical → rarely used in Tailwind
33,border-block-color,logical → use border-y-* instead
34,border-block-style,logical → use border-y-solid etc
35,border-block-width,logical → use border-y-4 etc
36,border-bottom,"border-b, border-b-4"
37,border-bottom-color,border-b-red-500
38,border-bottom-left-radius,rounded-bl-lg
39,border-bottom-right-radius,rounded-br-lg
40,border-bottom-style,"border-b-dashed, border-b-dotted"
41,border-bottom-width,"border-b-2, border-b-8"
42,border-collapse,"border-collapse, border-separate"
43,border-color,"border-inherit, border-current, border-red-500"
44,border-image,no direct (custom only)
45,border-image-outset,no direct
46,border-image-repeat,no direct
47,border-image-slice,no direct
48,border-image-source,no direct
49,border-image-width,no direct
50,border-inline,logical → use border-x instead
#,CSS Property,Tailwind Class (closest/real)
51,border-inline-color,use border-x-{color}
52,border-inline-style,"use border-x-solid, border-x-dashed etc"
53,border-inline-width,"use border-x, border-x-4, border-x-8 etc"
54,border-left,"border-l, border-l-4"
55,border-left-color,border-l-red-500
56,border-left-style,"border-l-dashed, border-l-dotted etc"
57,border-left-width,"border-l-2, border-l-8"
58,border-radius,"rounded, rounded-md, rounded-full, rounded-none"
59,border-right,"border-r, border-r-4"
60,border-right-color,border-r-blue-600
61,border-right-style,"border-r-dotted, border-r-double etc"
62,border-right-width,"border-r-2, border-r-8"
63,border-spacing,"border-spacing-x-4, border-spacing-y-8 (tables)"
64,border-style,"border-solid, border-dashed, border-none etc"
65,border-top,"border-t, border-t-4"
66,border-top-color,border-t-green-500
67,border-top-left-radius,"rounded-tl-lg, rounded-tl-3xl"
68,border-top-right-radius,"rounded-tr-lg, rounded-tr-3xl"
69,border-top-style,"border-t-dashed, border-t-double etc"
70,border-top-width,"border-t-2, border-t-8"
71,border-width,"border, border-0, border-2, border-4 etc"
72,bottom,"bottom-0, bottom-4, bottom-full, -bottom-10"
73,box-decoration-break,"box-decoration-clone, box-decoration-slice"
74,box-shadow,"shadow-sm, shadow, shadow-lg, shadow-xl, shadow-none"
75,box-sizing,"box-border, box-content"
76,break-after,"break-after-auto, break-after-avoid, break-after-page etc"
77,break-before,"break-before-auto, break-before-column etc"
78,break-inside,"break-inside-auto, break-inside-avoid"
79,caption-side,"caption-top, caption-bottom"
80,caret-color,"caret-red-500, caret-transparent etc"
81,clear,"clear-left, clear-right, clear-both, clear-none"
82,clip,deprecated – use clip-path instead
83,clip-path,"clip-path: inset(), clip-path: circle() → use ![clip-path:polygon(...)]"
84,color,"text-transparent, text-current, text-red-500 etc"
85,color-scheme,color-scheme: light dark → Tailwind v3.3+ prefers dark: variant
86,column-count,"columns-2, columns-3, columns-auto"
87,column-fill,"column-fill-balance, column-fill-auto"
88,column-gap,"gap-x-4, gap-x-12 (or just gap-4 in flex/grid)"
89,column-rule,"column-rule: 2px solid red → no direct, use custom"
90,column-rule-color,custom only
91,column-rule-style,custom only
92,column-rule-width,custom only
93,column-span,col-span-all (Tailwind v3.3+)
94,column-width,columns-3xs → columns-64 etc (fixed widths)
95,columns,"columns-3, columns-auto"
96,contain,"contain-none, contain-strict, contain-content etc (Tailwind v3.2+)"
97,content,"content-['hello'], content-none"
98,counter-increment,custom only (rare in Tailwind)
99,counter-reset,custom only
100,counter-set,custom only
#,CSS Property,Tailwind Class (closest/real)
101,cursor,"cursor-auto, cursor-pointer, cursor-wait, cursor-grab, cursor-not-allowed etc"
102,direction,"dir-ltr, dir-rtl (Tailwind v3.3+)"
103,display,"block, inline, flex, grid, hidden, table, contents etc"
104,empty-cells,"empty-cells-show, -empty-cells-hide"
105,filter,"blur(), brightness(), contrast(), drop-shadow(), grayscale(), sepia() etc"
106,flex,"flex, flex-1, flex-auto, flex-initial, flex-none"
107,flex-basis,"basis-0, basis-1/2, basis-full, basis-auto"
108,flex-direction,"flex-row, flex-row-reverse, flex-col, flex-col-reverse"
109,flex-flow,shorthand → use flex-row wrap etc separately
110,flex-grow,"grow, grow-0 (or flex-grow: 2 with arbitrary)"
111,flex-shrink,"shrink, shrink-0"
112,flex-wrap,"flex-wrap, flex-wrap-reverse, flex-nowrap"
113,float,"float-left, float-right, float-none"
114,font,"font-sans, font-mono, text-2xl, leading-tight etc"
115,font-family,"font-sans, font-serif, font-mono"
116,font-feature-settings,"arbitrary only → [font-feature-settings:""smcp""]"
117,font-kerning,"font-kerning-normal, font-kerning-none"
118,font-size,"text-xs, text-sm, text-4xl, text-[42px]"
119,font-smoothing,"antialiased, subpixel-antialiased"
120,font-style,"italic, not-italic"
121,font-variant,"small-caps, normal-nums, ordinal, slashed-zero"
122,font-weight,"font-thin, font-bold, font-black, font-[900]"
123,gap,"gap-4, gap-x-8, gap-y-12"
124,grid,shorthand → use separate grid classes
125,grid-area,grid-area: header → grid-[area:header] or named with template
126,grid-auto-columns,"auto-cols-min, auto-cols-max, auto-cols-fr"
127,grid-auto-flow,"grid-flow-row, grid-flow-col, grid-flow-dense"
128,grid-auto-rows,"auto-rows-min, auto-rows-max, auto-rows-fr"
129,grid-column,"col-span-4, col-start-2, col-end-6"
130,grid-column-end,"col-end-5, col-end-auto"
131,grid-column-start,"col-start-3, col-start-auto"
132,grid-row,"row-span-2, row-start-1, row-end-4"
133,grid-row-end,"row-end-3, row-end-auto"
134,grid-row-start,"row-start-2, row-start-auto"
135,grid-template,"shorthand → use grid-cols-4, grid-rows-3 etc"
136,grid-template-areas,"grid-areas-""header header"" ""main sidebar"" → arbitrary"
137,grid-template-columns,"grid-cols-12, grid-cols-[200px_1fr], grid-cols-subgrid"
138,grid-template-rows,"grid-rows-4, grid-rows-[100px_auto]"
139,hanging-punctuation,no direct Tailwind
140,height,"h-0, h-64, h-screen, h-full, h-[420px]"
141,hyphens,"hyphens-none, hyphens-manual, hyphens-auto"
142,image-rendering,"image-render-auto, image-render-pixel, image-render-crisp"
143,inset,"inset-0, inset-x-4, inset-y-full, top-0 right-0 bottom-0 left-0"
144,isolation,"isolate, isolation-auto"
145,justify-content,"justify-start, justify-center, justify-between, justify-around"
146,justify-items,"justify-items-start, justify-items-center, justify-items-stretch"
147,justify-self,"justify-self-auto, justify-self-start, justify-self-end"
148,left,"left-0, left-1/2, -left-4, left-[117px]"
149,letter-spacing,"tracking-tighter, tracking-wide, tracking-[0.2em]"
150,line-break,"break-normal, break-words, break-all, break-keep"
#,CSS Property,Tailwind Class (closest/real)
151,line-clamp,"line-clamp-1, line-clamp-3, line-clamp-none"
152,line-height,"leading-3, leading-tight, leading-[2.5]"
153,list-style,"list-none, list-disc, list-decimal"
154,list-style-image,list-image-[url(...)] arbitrary
155,list-style-position,"list-inside, list-outside"
156,list-style-type,"list-disc, list-decimal, list-georgian etc"
157,margin,"m-4, mx-auto, -mt-8, m-[117px]"
158,margin-block,"my-4, mb-8 (logical equivalent)"
159,margin-inline,"mx-6, ml-10 (logical equivalent)"
160,mask,"mask-image, mask-size → mostly arbitrary now"
161,max-height,"max-h-0, max-h-screen, max-h-[600px]"
162,max-width,"max-w-none, max-w-4xl, max-w-screen-2xl, max-w-[1170px]"
163,min-height,"min-h-0, min-h-screen, min-h-[400px]"
164,min-width,"min-w-0, min-w-full, min-w-[320px]"
165,mix-blend-mode,"mix-blend-multiply, mix-blend-screen, mix-blend-overlay etc"
166,object-fit,"object-contain, object-cover, object-fill, object-scale-down"
167,object-position,"object-center, object-top, object-bottom-left etc"
168,opacity,"opacity-0, opacity-75, opacity-[0.37]"
169,order,"order-1, order-first, order-last, order-12"
170,outline,"outline, outline-2, outline-dashed, outline-none"
171,outline-color,outline-red-500
172,outline-offset,outline-offset-4
173,outline-style,"outline-dotted, outline-double"
174,outline-width,"outline-0, outline-4"
175,overflow,"overflow-auto, overflow-hidden, overflow-x-scroll"
176,overflow-wrap,"break-words, break-normal"
177,overscroll-behavior,"overscroll-auto, overscroll-contain, overscroll-none"
178,padding,"p-8, px-4, py-12, pt-[117px]"
179,padding-block,py-6 (logical)
180,padding-inline,px-10 (logical)
181,perspective,"perspective-none, perspective-[1000px]"
182,place-content,"place-content-center, place-content-between"
183,place-items,"place-items-center, place-items-stretch"
184,place-self,"place-self-center, place-self-end"
185,pointer-events,"pointer-events-none, pointer-events-auto"
186,position,"static, relative, absolute, fixed, sticky"
187,resize,"resize, resize-none, resize-x, resize-y"
188,right,"right-0, right-1/2, -right-10"
189,rotate,"rotate-45, rotate-[117deg]"
190,scale,"scale-0, scale-150, scale-x-75, scale-[1.37]"
191,scroll-behavior,"scroll-auto, scroll-smooth"
192,scroll-margin,"scroll-m-8, scroll-mt-20"
193,scroll-padding,"scroll-p-4, scroll-px-12"
194,scroll-snap-align,"snap-start, snap-center, snap-end"
195,scroll-snap-type,"snap-x, snap-y, snap-both, snap-mandatory"
196,skew,"skew-x-6, skew-y-12, skew-[37deg]"
197,table-layout,"table-auto, table-fixed"
198,text-align,"text-left, text-center, text-right, text-justify"
199,text-decoration,"underline, line-through, no-underline, decoration-dashed"
200,text-decoration-color,decoration-red-500
#,CSS Property,Tailwind Class (closest/real)
151,line-clamp,"line-clamp-1, line-clamp-3, line-clamp-none"
152,line-height,"leading-3, leading-tight, leading-[2.5]"
153,list-style,"list-none, list-disc, list-decimal"
154,list-style-image,list-image-[url(...)] arbitrary
155,list-style-position,"list-inside, list-outside"
156,list-style-type,"list-disc, list-decimal, list-georgian etc"
157,margin,"m-4, mx-auto, -mt-8, m-[117px]"
158,margin-block,"my-4, mb-8 (logical equivalent)"
159,margin-inline,"mx-6, ml-10 (logical equivalent)"
160,mask,"mask-image, mask-size → mostly arbitrary now"
161,max-height,"max-h-0, max-h-screen, max-h-[600px]"
162,max-width,"max-w-none, max-w-4xl, max-w-screen-2xl, max-w-[1170px]"
163,min-height,"min-h-0, min-h-screen, min-h-[400px]"
164,min-width,"min-w-0, min-w-full, min-w-[320px]"
165,mix-blend-mode,"mix-blend-multiply, mix-blend-screen, mix-blend-overlay etc"
166,object-fit,"object-contain, object-cover, object-fill, object-scale-down"
167,object-position,"object-center, object-top, object-bottom-left etc"
168,opacity,"opacity-0, opacity-75, opacity-[0.37]"
169,order,"order-1, order-first, order-last, order-12"
170,outline,"outline, outline-2, outline-dashed, outline-none"
171,outline-color,outline-red-500
172,outline-offset,outline-offset-4
173,outline-style,"outline-dotted, outline-double"
174,outline-width,"outline-0, outline-4"
175,overflow,"overflow-auto, overflow-hidden, overflow-x-scroll"
176,overflow-wrap,"break-words, break-normal"
177,overscroll-behavior,"overscroll-auto, overscroll-contain, overscroll-none"
178,padding,"p-8, px-4, py-12, pt-[117px]"
179,padding-block,py-6 (logical)
180,padding-inline,px-10 (logical)
181,perspective,"perspective-none, perspective-[1000px]"
182,place-content,"place-content-center, place-content-between"
183,place-items,"place-items-center, place-items-stretch"
184,place-self,"place-self-center, place-self-end"
185,pointer-events,"pointer-events-none, pointer-events-auto"
186,position,"static, relative, absolute, fixed, sticky"
187,resize,"resize, resize-none, resize-x, resize-y"
188,right,"right-0, right-1/2, -right-10"
189,rotate,"rotate-45, rotate-[117deg]"
190,scale,"scale-0, scale-150, scale-x-75, scale-[1.37]"
191,scroll-behavior,"scroll-auto, scroll-smooth"
192,scroll-margin,"scroll-m-8, scroll-mt-20"
193,scroll-padding,"scroll-p-4, scroll-px-12"
194,scroll-snap-align,"snap-start, snap-center, snap-end"
195,scroll-snap-type,"snap-x, snap-y, snap-both, snap-mandatory"
196,skew,"skew-x-6, skew-y-12, skew-[37deg]"
197,table-layout,"table-auto, table-fixed"
198,text-align,"text-left, text-center, text-right, text-justify"
199,text-decoration,"underline, line-through, no-underline, decoration-dashed"
200,text-decoration-color,decoration-red-500