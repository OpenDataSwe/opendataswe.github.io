# Skatteberäkning

En enkel webb-applikation för att visa skatt.  

## Syfte

Att visa vilken skatt man betalar vid en given lön, och hur den skatten i sig är uppdelad.  
Samt, att förmedla hur skatten beräknas för att göra det enklare för den enskilde att förstå 
sin egen beskattning.  

## Bakgrund

Idag är det inte helt enkelt att räkna ut vilka skatter som gäller vid vilka inkomster, 
man kan använda [Ekonomifakta](https://www.ekonomifakta.se/), men då de har en stark politisk agenda
(de är en arbetsgivarorganisation) är det svårt att lita på information som sprids därifrån.  

Därav denna applikation, en eventuell politisk agenda spelar ingen roll, koden 
är öppen och ni kan dubbelkolla alla beräkningar om ni så önskar. Hittas ett fel så informera gärna, 
skapa en issue här på Github så korrigeras det.

## Arkitektur

Appen är skriven [yew](https://github.com/yewstack/yew) och blir där med en [wasm-modul](https://webassembly.org/), 
utöver den så är det bara ren [html](https://en.wikipedia.org/wiki/HTML), det är därför sidan är så 
estetiskt otilltalande.

### Uppdelning

Appen består av tre delar, ett lib: [skatt-lib](./skatt-lib), själva applikationskoden [skatt-fe](./skatt-fe), 
och en liten app som bara genererar en lista av sveriges kommuner [offline-runner](./offline-runner).  


## Data

Det är inte helt enkelt att förstå hur skatten ska beräknas, den här applikationens värden skiljer sig något 
mot Ekonomifaktas.  
Specifikt på hur jobbskatteavdrag räknas ut för inkomster under ca `10000:-` per månad. Varför det skiljer sig 
är ännu otydligt, någon räknar fel, mer på beräkningar senare. I fallet av jobbskatteavdrag
så räknar denna applikation ut ett högre avdrag än Ekonomifakta på låga inkomster.  

När inkomsterna överstiger `10000:-` per månad blir beräkningarna närmast identiska.

### Ålder

Ålder behöver anges för att ta reda på huruvida inkomsttagaren fyllt 66 år vid årets ingång, 
vissa skatteregler skiljer sig för dessa.  

### Kommun

Kommun behöver anges då de kommunala skattesatserna ska kunna hämtas från skatteverkets api. 

APIet kan hittas [här](https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun).  

### Medlem i Svenska kyrkan

Om man är medlem i Svenska kyrkan betalar man i regel kyrkoavgift. Om rutan för 
medlemskap kryssas i måste även rätt församling väljas för skatteberäkningen ska bli korrekt.

Läs mer om kyrkoavgiften [här](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/kyrkoavgift.4.3152d9ac158968eb8fd2db3.html).

Datat för skattesats per församling hämtas från [skatteverkets api](https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun).  

## Grafer

I skrivande stund kan datat visualiseras på sex olika sätt.  

### Linjediagram

Fyra av visualiseringarna är linjediagram.  

#### Start

Välj start-inkomst att börja beräkna skatt från, det är månatlig inkomst.  

#### Steg

Välj steg mellan beräkningar. För t.ex. `100` räknas skatten ut för en månatlig inkomst på 
`100, 200, 300, ...` SEK. Lägre steg gör beräkningen tyngre (tar längre tid), går stegen under 100 
blir även beräkningen något tvivelaktig.  

Skatteverkets grundavdrag ges som spann med `100:-`-glipor mellan. Det innebär att 
grundavdraget kan vara definierat för t.ex. `16000-17000`, och `17100-18100` utan att säga något 
om vad grundavdraget borde vara för `17050:-`. Det är otydligt huruvida beräkningen borde ske med det högre 
eller lägre grundavdraget, det högre väljs här utan vidare motivering.  

#### Slut

Slutinkomst för att beräkna skattesumman, det är månatlig inkomst.  

#### Procentgraf

En graf som visar procent skatt över krona tjänad, olika start och slut-löner, samt steg kan väljas.

Ytterligare kan vad som visas i grafen väljas genom att klicka i rutorna.

#### Procentgraf lutning

En beräkning av lutningen mellan punker av den procentuella skatten för lönespannet.  

Det är bara `(y2 - y1) / (x2 - x1)`, det vill säga den procentuella skillnaden mellan två löner, genom 
skillnaden i kronor mella samma två löner.  

Det är ekvivalent med [marginalskatten](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/sabeskattasdinlon/marginalskatt.4.61589f801118cb2b7b280006375.html), 
fast i en svåravläslig form, se 'Absolutgraf lutning' för marginalskatten som den brukar visas.  

#### Absolutgraf

En graf som visar skatt i kronor över krona tjänad, olika start och slut-löner, samt steg kan väljas.

Ytterligare kan vad som visas i grafen väljas genom att klicka i rutorna.

#### Absolutgraf lutning

En beräkning av lutningen mellan punker av den absoluta skatten för lönespannet.  

Det är bara `(y2 - y1) / (x2 - x1)`, det vill säga den absoluta skillnaden i kronor mellan två löner, genom 
skillnaden i kronor mella samma två löner.  

Det är ekvivalent med [marginalskatten](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/sabeskattasdinlon/marginalskatt.4.61589f801118cb2b7b280006375.html).  

### Cirkeldiagram

Cirkeldiagrammet visar skattefördelningen. Vid lägre löner är det bara arbetsgivaravgift i pajen, men 
när lönen ökar kommer andra skatter, så som kommunal- och till slut statlig inkomstskatt ta upp delar av den.  

### Tabell

En tabell som visar de olika posterna som utgör skatten i procent och kronor.  

## Beräkningar

Skatten består av ett antal delar.  

### Kommunal skatt

De kommunala skatterna består av fyra delar, totalen beräknas genom summering.  

Datat för alla fyra delar kommer från 
[skatteverkets api](https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun).

#### Kyrkoavgift

Om inkomsttagaren är medlem i en församling betalar denne kyrkskatt, den varierar per församling.

Den ligger ofta kring `1%`.

Läs på mer [här](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/kyrkoavgift.4.3152d9ac158968eb8fd2db3.html).

#### Begravningsavgift

Begravningsavgift betalas av alla som är folkbokförda i Sverige. 

Läs på mer [här](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/begravningsavgift.4.1ee2ea81054cf37b1c800030.html)

#### Landstingsskatt/Regionsskatt

Pengar som går till regionen. 

Läs mer [här](https://www.skatteverket.se/offentligaaktorer/utbetalningavskatt.4.361dc8c15312eff6fd1a79d.html)

#### Kommunalskatt

Pengar som går till kommunen.

Läs mer [här](https://www.skatteverket.se/offentligaaktorer/utbetalningavskatt.4.361dc8c15312eff6fd1a79d.html)(samma som ovan)

### Statlig inkomstskatt

[Statlig inkomstskatt](https://www.skatteverket.se/privat/etjansterochblanketter/svarpavanligafragor/inkomstavtjanst/privattjansteinkomsterfaq/narskamanbetalastatliginkomstskattochhurhogarden.5.10010ec103545f243e8000166.html) 
betalar man om man tjänar en bit över den så kallade skiktgränsen.  

Men, eftersom [grundavdraget](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html) går först börjar man inte betala statlig inkomstskatt förrän man 
tjänar en summa som överstiger `skiktgräns - grundavdrag`.  

Beräknas genom att ta `20%` av lönen som överstiger `skiktgräns - grundavdrag`.

Läs mer [här](https://www.skatteverket.se/privat/etjansterochblanketter/svarpavanligafragor/inkomstavtjanst/privattjansteinkomsterfaq/narskamanbetalastatliginkomstskattochhurhogarden.5.10010ec103545f243e8000166.html)

### Public service-avgift

Public service-avgiften betalas av alla som tjänar över [grundavdraget](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html).

Tjänar man `121 919:-` eller lägre betalar man 1% av inkomsten, annars betalar man maxgränsen `1219:-`.  

Läs mer [här](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/publicserviceavgift.4.22501d9e166a8cb399f31dd.html)

### Grundavdrag

Grundavdraget dras från inkomsterna från gällande år, i regel går skatten på det återstående beloppet.

Skiljer sig beroende på inkomst och ålder.

Gällande grundavdrag hämtas från Skatteverkets API 
[här för de under 66](https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Grundavdrag%20f%C3%B6r%20personer%20under%2066%20%C3%A5r), 
och [här för de 66 och äldre](https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Grundavdrag%20f%C3%B6r%20personer%2066%20%C3%A5r%20och%20%C3%A4ldre).  

Läs mer [här](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html)

### Jobbskatteavdrag

Jobbskatteavdraget är ett avdrag som vissa inkomsttagare får göra på sin skatt, 
och det mest komplicerade att räkna ut.  

Se [denna länk](https://www4.skatteverket.se/rattsligvagledning/edition/2024.3/2940.html), 
alternativt [denna länk](https://www4.skatteverket.se/rattsligvagledning/27071.html?date=2024-06-13) för detaljerna.

### Skattereduktion förvärvsinkomst

Om man tjänar mer än `40 000:-` om året kan man få en skattereduktion upp till `1500:-`.

Beräknas genom att undersöka inkomsten:

Är den mellan `40 000:-` och `240 000:-` får inkomsttagaren `0.75%` av inkomsten i skattereduktion.

Överstiger den `240 000:-` får inkomsttagaren `1500:-`.  

Läs mer [här](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/skattereduktioner.4.3810a01c150939e893f1a17e.html)


### Total personlig skatt

Summan av alla skatter som betalas av inkomsttagaren själv, det är alla skatter och avdrag exclusive 
arbetsgivaravgiften.  


### Arbetsgivaravgift

Arbetsgivaravgiften betraktas av vissa som skatt och tas därmed med här, den betalas av arbetsgivaren 
och baseras på bruttolönen.

För personer som vid årets ingång into fyllt 66 år är det `31.24%`, för personer 
som inte fyllt 66 år vid årets ingång är det `10.21%`.  

Läs mer [här](https://www.skatteverket.se/foretag/arbetsgivare/arbetsgivaravgifterochskatteavdrag/arbetsgivaravgifter.4.233f91f71260075abe8800020817.html)

### Total skattesumma

Summan av alla skatter som betalas av inkomsttagaren själv, alla skatter och avdrag, inklusive 
arbetsgivaravgiften.  

### Kvar efter skatt

Pengarna som arbetstagaren får ut efter skatt, `bruttolön - personliga skatter`.  
