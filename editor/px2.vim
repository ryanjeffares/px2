if exists('b:current_syntax')
  finish
endif

syn keyword px2Keywords dup drop over swap rot println skipwhite
syn keyword px2Booleans true false skipwhite

syn match px2Number "\v<\d+>"

hi def link px2Keywords   Keyword
hi def link px2Booleans   Boolean
hi def link px2Number     Number
