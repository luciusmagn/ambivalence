.\" mk slides.show
.de IT
.if !"\\$1"" .ds BX \\$1
.if "\\$1"" .ds BX \(bu
.IP \\*(BX
..
.de SS
.ds CH \s+2\f3\\$1\fP\s-2
.bp
.LP
.na
..
.nr PO 0.5i
.nr HM 1i
.nr PS 12
.nr VS 14p
.nr PD 6p
.nr LL 6.5i
.ds CH "
.pl 15c
.ds RF "
.TL
Titel
.AU
The Author
.AI
The Mail
.PP
.SS "Slide Header
.IT
Point
.IT
Point
.SS "Next Slide
.IT
next Point!
