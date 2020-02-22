# 9 kruhů pekla
## Elektronické sledování


#### Lukáš Hozda
---

# Oops, Dante

---

# Abstrakce?

---

|                |
| -------------- |
| Hardware 		 |
| Firmware 		 |
| Systém 		 |
| Aplikace 		 |
| Sítě 			 |
| Stránky 		 |
| Komprehensivní |
| Korporátní 	 |
| Vládní 		 |


("kruhy" pronikají směrem nahoru)

---

> "You have to fight for your privacy or you lose it."

<div style="text-align: right">
- Eric Schmidt
</div>

---

<div style="text-align: center">
Potenciál sledovat a narušovat soukromí uživatelů je stejně špatný jako narušování samé,
protože tam kde existuje potenciál, nemůžeme potvrdit, že ke sledování nedochází.
</div>

---

## Hardware

---

- Zadní vrátka v hardware, ať už od výrobce nebo od třetí strany
- Detekce je poměrně složitá, najisto pouze "mikroskopem"

---

### Hardwarový trojan

---

- zařízení, které poškozuje funkci nebo důvěryhodnost elektronického systému
- může se jednat o čip, primitivní elektrický okruh nebo celý přístroj
- původci různí: třetí strany, výrobci i vládní organizace

---

## Kauza čínských špionážních čipů
##### 2018

---

![ultra mega špijón]( https://liberte.pl/app/uploads/2018/10/600x-1-e1538732898397-1140x0-c-default.jpg )

---

## Intel Management Engine
#### (a AMD PSP)

---

### Špijón v procesoru

![hacker inside](https://tr3.cbsistatic.com/hub/i/r/2016/05/09/d0e68f1d-e3d3-4770-a9b4-a508c9d2c8d5/resize/1200x/fd768dbb21e95c41b931ecd7f49b30a7/38hackerinside.jpg)

---

### Obecně se hardwareové útoky pro většinu aplikací nevyplatí...

---

## Firmware

---

- jedná se o typ software pro nízko-úrovňovou kontrolu hardware
- nezávislý na operačním systému
- uložený na oddělené paměti, někdy přímo v zařízení
- BIOS

---

## Bioskit (otrávený hypervizor)

---

- virus který se skryje do BIOSu a má kompletní kontrolu nad počítačem
- prakticky nedetekovatelný bez složitého zásahu do zařízení
- Trojan.Bioskit.1 (infikuje počítače s čínskými antiviry)

---

## Průšvih Adups (otrávený firmware Androidů)
##### 2016

---

### 700 miliónů postižených zařízení

![adups](http://www.adups.com/wenjinload/img/201610261401297977.png)

---

## Apple Dark Matter
##### 2017

---

- Sonic Screwdriver (Macy, TB3 port)
- Nightskies        (iPhony)
- Trident           (iPhony, ne kvůli CIA)
	- <small>2016</small>

---

## Operační Systémy

---

## Microsoft Windows

---

![not spying](https://fossbytes.com/wp-content/uploads/2015/09/windows_10_not-spying-microsoft-says.jpg)

> "Víme všechno, ale je to pro vaše dobro."
<div style="text-align: right">
<small style="font-size: 70%">- Někdo v Microsoftu lol</small>
</div>

---

## Apple macOS

---

![not spying too lol](https://www.pngitem.com/pimgs/m/117-1176387_mac-os-logo-vector-hd-png-download.png)

> "My toho víme ještě víc, ale aspoň se to dá vypnout"
<small style="font-size: 70%">- nějakej týpek z Applu</small>

---

## Ani telefony nejsou v pořádku

---

![iosadroid](https://pngimage.net/wp-content/uploads/2018/06/ios-android-logo-png-1.png)

> "Think Different - __Spyware__."

<small style="text-align: right; font-size: 50%">Guardian (Siri)/Forbes - iOS, různé - Android)</small>

---

## Aplikace

---


### většina velkých programů

---

### mobilní aplikace

---
### software klávesnice

---

### prohlížeče

---

## A propos prohlížeče...

---

## Internet

![my intenret](https://www.quotemaster.org/images/fa/fab925609361102f4a2efe8e7d2b8d92.jpg)

---

### Milujeme veřejné sítě a neznalé uživatele

---

- Man-in-the-Middle attack
- Rogue hotspot
- sledování autoritou

---

### Man-in-the-Middle attack
![in the middle with you](https://thehacktoday.com/wp-content/uploads/2019/10/man-in-the-middle.png)

---

### Rogue hotspot
![rogue hotspot](https://2.bp.blogspot.com/-_XiV8lPmTuU/WZzT2FkcU2I/AAAAAAAAFxk/s0C5tpyphFMSAcbeVu2GWk4IQCnRU2b6gCLcBGAs/s1600/Grafik-Rouge-Hotspot.jpg)

---

### Sledování autoritou

- každý provozovatel (ať už lokální sítě nebo poskytovatel připojení) může sledovat provoz
- u proprietárních firmwarů pro routery neexistuje jistota, že router nezneuživá data

---

## Stránky

---

- HTTP (User-Agent), štěnice, Super cookies, fingerprinting
- reklamy, heat mapy, zabudovaný obsah

---

### HTTP

```bash
Host: developer.mozilla.org
User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0
Accept: text/css,*/*;q=0.1
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate, br
DNT: 1
Connection: keep-alive
Cookie: _ga=GA1.2.1813597394.1575138761; dwf_sg_task_completion=False;
       lux_uid=158012581475482170
```

---

### Reklamy

- většina reklam má přístup k prostředí, ve kterém je zobrazovaná
- AdBlock se vyplatí, i když člověku reklamy nepřipadají audiovizuálně otravné

---

### Zabudovaný obsah

- sledování přes tlačítka, obrázky a spol.
- reCaptcha
- pokročilejší: sledování přes styly

---

### Kauza pluginů
##### 2014

- prodávání, adware, skryté sledování historie a chování

<small style="font-size: 40%">Lowell Heddings</small>

---

## Komprehensivní sledování

---

## Google, Facebook, Microsoft

---

- shromažďování obrovského množství dat skrze služby, vyhledávání, reklamy, aktivity a spol.
- nestačí se odhlásit - speciální cookies, korelace, "anonymní" identifikátory, otisky prstů

---

## Google ví všechno

---

- Kdo jste
- Co vás zajímá a jaké jsou vaše preference
- Profesionální zaměření, finanční situace
- Historie míst/pobytů
- Kdo je vaše rodina a přátelé
- Na jaké se koukáte video
- Politický názor
- Veškerá online aktivita

---

## Facebook taky

---

- Kdo jste vy, vaše rodiny a vaši přátelé
- Koho, kdy, jak dlouho voláte, nebo s kým komunikujete
- Telefonní číslo, kontakty
- Zájmy, názory, vzdělání, vztahy
- Náboženství
- Místa přístupu, fyzické lokace
- a další...
- Instagram, Whatsapp..

---

## Korporátní sledování

---

- velké firmy mají zájem sledovat své zaměstnance

---

### V pracovní době

- Používání internetu a aplikací
- Emaily
- Obrazovka počítače
- Služební telefony
- GPS sledování služebních vozů
- Pohyb po pracovišti pomocí čipových karet

---

### Mimo pracovní dobu

- Procházení sociálních médií
- Pokusy o sledování přes firemní aplikace
- Zaznamenávání stisků kláves

---

## Vládní sledování

---

## Rusko, Čína, S. Korea

---

### Rusko

---

- Odposlouchává hovory ↓
- "Lawful intercept" - zákonné zachytávání
- Pokus o vlastní internet, který by byl pod kontrolou vlády (fail)

---

### Čína

---

- Kamery všude (4:1)
- Alternativy pro internetové služby (Baidu a spol., QQ)
- Kompletní kontrola dat procházejících sítí
- Elektronické kádrové profily
- Velký firewall - kontrola obsahu

---

### Severní Korea

---

- Uzavřená síť
- Vlastní operační systém
- Všechna data dostupná
- Sledovatelná eleketronika není ještě tak častá v SK

---

### Velká Británie

---

- Sledování sociálních médií -> zatýkání
- Specializované týmy na online sledování
- Automatické rozpoznávání obličejů

---

## Závěr

---

# Je to na nic

---

# Děkuji za pozornost

---

### Zdroje

https://www.cookiepro.com/blog/website-tracking/
https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers
https://hackaday.com/2017/12/11/what-you-need-to-know-about-the-intel-management-engine/
https://securityboulevard.com/2018/11/guarding-against-backdoors-and-malicious-hardware/
https://www.tomshardware.com/news/malwarebytes-finds-malware-in-settings-app-on-cheap-phones
https://www.techdesignforums.com/practice/guides/hardware-trojan-security-countermeasures/
https://www.pcmag.com/news/does-your-motherboard-have-a-secret-chinese-spy-chip
https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet#images
https://www.techopedia.com/10-quotes-about-tech-privacy-thatll-make-you-think/2/33713
https://www.secureworldexpo.com/industry-news/top-20-cybersecurity-quotes
https://www.benthamsgaze.org/2018/02/06/a-witch-hunt-for-trojans-in-our-chips/
https://news.drweb.com/show/?i=1879
http://swarup.ece.ufl.edu/papers/IC/IC6.pdf
https://bgr.com/2016/12/20/adups-spyware-android-phones-security/
https://www.inverse.com/culture/louis-vuitton-2020-lookbook
https://www.cnbc.com/2017/11/17/how-to-find-out-what-facebook-knows-about-me.html
https://www.worktime.com/is-it-legal-and-ethical-to-use-employee-computer-monitoring-software
https://www.computerhope.com/issues/ch001501.htm
https://www.worktime.com/is-it-legal-and-ethical-to-use-employee-computer-monitoring-software
https://windowsreport.com/windows-10-spyware-telemetry-task-scheduler/
https://en.wikipedia.org/wiki/Mass_surveillance_in_China
https://www.independent.co.uk/news/uk/politics/uk-mass-surveillance-gchq-eu-human-rights-echr-edward-snowden-a8535571.html
https://www.spectator.co.uk/2016/08/what-happened-when-i-was-charged-with-a-hate-crime/
https://www.theatlantic.com/international/archive/2018/02/china-surveillance/552203/
https://en.wikipedia.org/wiki/Mass_surveillance_in_North_Korea
https://eu.usatoday.com/story/tech/2019/10/08/is-the-boss-tracking-you-now/3901594002/
https://www.fastcompany.com/90318167/workplace-tracking-is-growing-fast-most-workers-dont-seem-very-concerned
https://www.marketwatch.com/story/your-employer-is-tracking-your-every-move-is-it-too-late-to-do-anything-about-it-2019-07-24
https://www.wsj.com/articles/the-new-ways-your-boss-is-spying-on-you-11563528604
https://en.wikipedia.org/wiki/Lawful_interception


