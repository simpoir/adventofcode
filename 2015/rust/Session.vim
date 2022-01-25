let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Source/adventofcode/2021/rust
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
argglobal
%argdel
$argadd ~/Source/adventofcode/2015/rust/src/main.rs
edit ~/Source/adventofcode/2015/rust/src/cli.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
wincmd _ | wincmd |
vsplit
2wincmd h
wincmd w
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 85 + 121) / 242)
exe 'vert 2resize ' . ((&columns * 80 + 121) / 242)
exe 'vert 3resize ' . ((&columns * 75 + 121) / 242)
argglobal
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 1 - ((0 * winheight(0) + 28) / 56)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 017|
lcd ~/Source/adventofcode/2015/rust
wincmd w
argglobal
if bufexists("~/Source/adventofcode/2015/rust/build.rs") | buffer ~/Source/adventofcode/2015/rust/build.rs | else | edit ~/Source/adventofcode/2015/rust/build.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Source/adventofcode/2015/rust/build.rs
endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 8 - ((7 * winheight(0) + 28) / 56)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 8
normal! 0
lcd ~/Source/adventofcode/2015/rust
wincmd w
argglobal
if bufexists("~/Source/adventofcode/2015/rust/src/main.rs") | buffer ~/Source/adventofcode/2015/rust/src/main.rs | else | edit ~/Source/adventofcode/2015/rust/src/main.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Source/adventofcode/2015/rust/src/main.rs
endif
balt ~/Source/adventofcode/2015/rust/src/cli.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 14 - ((13 * winheight(0) + 28) / 56)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 14
normal! 024|
lcd ~/Source/adventofcode/2015/rust
wincmd w
exe 'vert 1resize ' . ((&columns * 85 + 121) / 242)
exe 'vert 2resize ' . ((&columns * 80 + 121) / 242)
exe 'vert 3resize ' . ((&columns * 75 + 121) / 242)
tabnext 1
badd +13 ~/Source/adventofcode/2015/rust/src/main.rs
badd +0 ~/Source/adventofcode/2021/rust/src/main.rs
badd +2 ~/Source/adventofcode/2015/rust/src/util.rs
badd +8 ~/Source/adventofcode/2015/rust/src/cli.rs
badd +6 ~/Source/adventofcode/2015/rust/build.rs
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 shortmess=filnxtToOFI
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
