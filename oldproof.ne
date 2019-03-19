Na levé straně můžeme nechat $z$ jako všechny otočené pevné bity, na pravé straně musíme dosadit do konkrétní rovnice za každé $z$ výraz
.EQ
(z - z " mod " 2 sup {n sub pred - 1 + m sub min}) + 2 sup {n sub pred - 1 + m sub min}
.EN
Tedy:
.EQ
R = "\fR{" [(z - z " mod " 2 sup {n sub pred - 1 + m sub min}) + 2 sup {n sub pred - 1 + m sub min}] - [(z - z " mod " 2 sup {n sub pred - 1 + m sub min}) + 2 sup {n sub pred - 1 + m sub min}] " mod " 2 sup {n sub pred - 1 + m sub min} "\fR}" +
.EN
.EQ
+ 2 sup {n sub pred - 1 + m sub min}
.EN
Celá část $[(z - z " mod " 2 sup {n sub pred - 1 + m sub min}) + 2 sup {n sub pred - 1 + m sub min}] " mod " 2 sup {n sub pred - 1 + m sub min}$ se dá kompletně "vykrátit". Zbude toto:
.bp
.EQ
R = z - z " mod " 2 sup {n sub pred - 1 + m sub min} + 2 sup {n sub pred - 1 + m sub min} + 2 sup {n sub pred - 1 + m sub min}
.EN
