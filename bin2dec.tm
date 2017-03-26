# Turing machine to convert binary to decimal with a single state

[s]:f,t
[a]:O,I,0,1,2,3,4,5,6,7,8,9,_,.,~,&

[e]:f
[x]:t

[t|f]:I->(f,O,<)|O->(f,.,>)|.->(f,I,<)|_->(f,1,>)|1->(f,2,>)|2->(f,3,>)|3->(f,4,>)|4->(f,5,>)|5->(f,6,>)|6->(f,7,>)|7->(f,8,>)|8->(f,9,>)|9->(f,&,<)|&->(f,0,>)|0->(f,1,>)|~->(t,~,>)

[b|_]:___[I]IIOIIII~~~~~

