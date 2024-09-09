//! Generated code, do not edit

pub const ABSOLUTGRAF: &str = "<h4>Absolutgraf</h4>
<p>En graf som visar skatt i kronor över krona tjänad, olika start och slut-löner, samt steg kan väljas.</p>
<p>Ytterligare kan vad som visas i grafen väljas genom att klicka i rutorna.</p>
";

pub const ABSOLUTGRAF_LUTNING: &str = r#"<h4>Absolutgraf lutning</h4>
<p>En beräkning av lutningen mellan punker av den absoluta skatten för lönespannet.</p>
<p>Det är bara <code>(y2 - y1) / (x2 - x1)</code>, det vill säga den absoluta skillnaden i kronor mellan två löner, genom
skillnaden i kronor mella samma två löner.</p>
<p>Det är ekvivalent med <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/sabeskattasdinlon/marginalskatt.4.61589f801118cb2b7b280006375.html">marginalskatten</a>.</p>
"#;

pub const ARBETSGIVARAVGIFT: &str = r#"<h3>Arbetsgivaravgift</h3>
<p>Arbetsgivaravgiften betraktas av vissa som skatt och tas därmed med här, den betalas av arbetsgivaren
och baseras på bruttolönen.</p>
<p>För personer som vid årets ingång into fyllt 66 år är det <code>31.24%</code>, för personer
som inte fyllt 66 år vid årets ingång är det <code>10.21%</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/foretag/arbetsgivare/arbetsgivaravgifterochskatteavdrag/arbetsgivaravgifter.4.233f91f71260075abe8800020817.html">här</a></p>
"#;

pub const ARKITEKTUR: &str = r#"<h2>Arkitektur</h2>
<p>Appen är skriven <a href="https://github.com/yewstack/yew">yew</a> och blir där med en <a href="https://webassembly.org/">wasm-modul</a>,
utöver den så är det bara ren <a href="https://en.wikipedia.org/wiki/HTML">html</a>, det är därför sidan är så
estetiskt otilltalande.</p>
<h3>Uppdelning</h3>
<p>Appen består av tre delar, ett lib: <a href="./skatt-lib">skatt-lib</a>, själva applikationskoden <a href="./skatt-fe">skatt-fe</a>,
och en liten app som bara genererar en lista av sveriges kommuner <a href="./offline-runner">offline-runner</a>.</p>
"#;

pub const BAKGRUND: &str = r#"<h2>Bakgrund</h2>
<p>Idag är det inte helt enkelt att räkna ut vilka skatter som gäller vid vilka inkomster,
man kan använda <a href="https://www.ekonomifakta.se/">Ekonomifakta</a>, men då de har en stark politisk agenda
(de är en arbetsgivarorganisation) är det svårt att lita på information som sprids därifrån.</p>
<p>Därav denna applikation, en eventuell politisk agenda spelar ingen roll, koden
är öppen och ni kan dubbelkolla alla beräkningar om ni så önskar. Hittas ett fel så informera gärna,
skapa en issue här på Github så korrigeras det.</p>
"#;

pub const BEGRAVNINGSAVGIFT: &str = r#"<h4>Begravningsavgift</h4>
<p>Begravningsavgift betalas av alla som är folkbokförda i Sverige.</p>
<p>Läs på mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/begravningsavgift.4.1ee2ea81054cf37b1c800030.html">här</a></p>
"#;

pub const BERAKNINGAR: &str = r#"<h2>Beräkningar</h2>
<p>Skatten består av ett antal delar.</p>
<h3>Kommunal skatt</h3>
<p>De kommunala skatterna består av fyra delar, totalen beräknas genom summering.</p>
<p>Datat för alla fyra delar kommer från
<a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun">skatteverkets api</a>.</p>
<h4>Kyrkoavgift</h4>
<p>Om inkomsttagaren är medlem i en församling betalar denne kyrkskatt, den varierar per församling.</p>
<p>Den ligger ofta kring <code>1%</code>.</p>
<p>Läs på mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/kyrkoavgift.4.3152d9ac158968eb8fd2db3.html">här</a>.</p>
<h4>Begravningsavgift</h4>
<p>Begravningsavgift betalas av alla som är folkbokförda i Sverige.</p>
<p>Läs på mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/begravningsavgift.4.1ee2ea81054cf37b1c800030.html">här</a></p>
<h4>Landstingsskatt/Regionsskatt</h4>
<p>Pengar som går till regionen.</p>
<p>Läs mer <a href="https://www.skatteverket.se/offentligaaktorer/utbetalningavskatt.4.361dc8c15312eff6fd1a79d.html">här</a></p>
<h4>Kommunalskatt</h4>
<p>Pengar som går till kommunen.</p>
<p>Läs mer <a href="https://www.skatteverket.se/offentligaaktorer/utbetalningavskatt.4.361dc8c15312eff6fd1a79d.html">här</a>(samma som ovan)</p>
<h3>Statlig inkomstskatt</h3>
<p><a href="https://www.skatteverket.se/privat/etjansterochblanketter/svarpavanligafragor/inkomstavtjanst/privattjansteinkomsterfaq/narskamanbetalastatliginkomstskattochhurhogarden.5.10010ec103545f243e8000166.html">Statlig inkomstskatt</a>
betalar man om man tjänar en bit över den så kallade skiktgränsen.</p>
<p>Men, eftersom <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html">grundavdraget</a> går först börjar man inte betala statlig inkomstskatt förrän man
tjänar en summa som överstiger <code>skiktgräns - grundavdrag</code>.</p>
<p>Beräknas genom att ta <code>20%</code> av lönen som överstiger <code>skiktgräns - grundavdrag</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/etjansterochblanketter/svarpavanligafragor/inkomstavtjanst/privattjansteinkomsterfaq/narskamanbetalastatliginkomstskattochhurhogarden.5.10010ec103545f243e8000166.html">här</a></p>
<h3>Public service-avgift</h3>
<p>Public service-avgiften betalas av alla som tjänar över <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html">grundavdraget</a>.</p>
<p>Tjänar man <code>121 919:-</code> eller lägre betalar man 1% av inkomsten, annars betalar man maxgränsen <code>1219:-</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/publicserviceavgift.4.22501d9e166a8cb399f31dd.html">här</a></p>
<h3>Grundavdrag</h3>
<p>Grundavdraget dras från inkomsterna från gällande år, i regel går skatten på det återstående beloppet.</p>
<p>Skiljer sig beroende på inkomst och ålder.</p>
<p>Gällande grundavdrag hämtas från Skatteverkets API
<a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Grundavdrag%20f%C3%B6r%20personer%20under%2066%20%C3%A5r">här för de under 66</a>,
och <a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Grundavdrag%20f%C3%B6r%20personer%2066%20%C3%A5r%20och%20%C3%A4ldre">här för de 66 och äldre</a>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html">här</a></p>
<h3>Jobbskatteavdrag</h3>
<p>Jobbskatteavdraget är ett avdrag som vissa inkomsttagare får göra på sin skatt,
och det mest komplicerade att räkna ut.</p>
<p>Se <a href="https://www4.skatteverket.se/rattsligvagledning/edition/2024.3/2940.html">denna länk</a>,
alternativt <a href="https://www4.skatteverket.se/rattsligvagledning/27071.html?date=2024-06-13">denna länk</a> för detaljerna.</p>
<h3>Skattereduktion förvärvsinkomst</h3>
<p>Om man tjänar mer än <code>40 000:-</code> om året kan man få en skattereduktion upp till <code>1500:-</code>.</p>
<p>Beräknas genom att undersöka inkomsten:</p>
<p>Är den mellan <code>40 000:-</code> och <code>240 000:-</code> får inkomsttagaren <code>0.75%</code> av inkomsten i skattereduktion.</p>
<p>Överstiger den <code>240 000:-</code> får inkomsttagaren <code>1500:-</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/skattereduktioner.4.3810a01c150939e893f1a17e.html">här</a></p>
<h3>Total personlig skatt</h3>
<p>Summan av alla skatter som betalas av inkomsttagaren själv, det är alla skatter och avdrag exclusive
arbetsgivaravgiften.</p>
<h3>Arbetsgivaravgift</h3>
<p>Arbetsgivaravgiften betraktas av vissa som skatt och tas därmed med här, den betalas av arbetsgivaren
och baseras på bruttolönen.</p>
<p>För personer som vid årets ingång into fyllt 66 år är det <code>31.24%</code>, för personer
som inte fyllt 66 år vid årets ingång är det <code>10.21%</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/foretag/arbetsgivare/arbetsgivaravgifterochskatteavdrag/arbetsgivaravgifter.4.233f91f71260075abe8800020817.html">här</a></p>
<h3>Total skattesumma</h3>
<p>Summan av alla skatter som betalas av inkomsttagaren själv, alla skatter och avdrag, inklusive
arbetsgivaravgiften.</p>
<h3>Kvar efter skatt</h3>
<p>Pengarna som arbetstagaren får ut efter skatt, <code>bruttolön - personliga skatter</code>.</p>
"#;

pub const CIRKELDIAGRAM: &str = "<h3>Cirkeldiagram</h3>
<p>Cirkeldiagrammet visar skattefördelningen. Vid lägre löner är det bara arbetsgivaravgift i pajen, men
när lönen ökar kommer andra skatter, så som kommunal- och till slut statlig inkomstskatt ta upp delar av den.</p>
";

pub const DATA: &str = r#"<h2>Data</h2>
<p>Det är inte helt enkelt att förstå hur skatten ska beräknas, den här applikationens värden skiljer sig något
mot Ekonomifaktas.<br />
Specifikt på hur jobbskatteavdrag räknas ut för inkomster under ca <code>10000:-</code> per månad. Varför det skiljer sig
är ännu otydligt, någon räknar fel, mer på beräkningar senare. I fallet av jobbskatteavdrag
så räknar denna applikation ut ett högre avdrag än Ekonomifakta på låga inkomster.</p>
<p>När inkomsterna överstiger <code>10000:-</code> per månad blir beräkningarna närmast identiska.</p>
<h3>Ålder</h3>
<p>Ålder behöver anges endast för att ta reda på huruvida inkomsttagaren fyllt 66 år vid årets ingång,
vissa skatteregler skiljer sig för dessa.</p>
<h3>Kommun</h3>
<p>Kommun behöver anges då de kommunala skattesatserna ska kunna hämtas från skatteverkets api.</p>
<p>Apiet kan hittas <a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun">här</a>.</p>
<h3>Medlem i Svenska kyrkan</h3>
<p>Om man är medlem i Svenska kyrkan betalar man i regel kyrkoavgift. Om rutan för
medlemskap kryssas i måste även rätt församling väljas för skatteberäkningen ska bli korrekt.</p>
<p>Läs mer om kyrkoavgiften <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/kyrkoavgift.4.3152d9ac158968eb8fd2db3.html">här</a>.</p>
<p>Datat för skattesats per församling hämtas från <a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun">skatteverkets api</a>.</p>
"#;

pub const GRAFER: &str = r#"<h2>Grafer</h2>
<p>I skrivande stund kan datat visualiseras på sex olika sätt.</p>
<h3>Linjediagram</h3>
<p>Fyra av visualiseringarna är linjediagram.</p>
<h4>Start</h4>
<p>Välj start-inkomst att börja beräkna skatt från, det är månatlig inkomst.</p>
<h4>Steg</h4>
<p>Välj steg mellan beräkningar. För t.ex. <code>100</code> räknas skatten ut för en månatlig inkomst på
<code>100, 200, 300, ...</code> SEK. Lägre steg gör beräkningen tyngre (tar längre tid), går stegen under 100
blir även beräkningen något tvivelaktig.</p>
<p>Skatteverkets grundavdrag ges som spann med <code>100:-</code>-glipor mellan. Det innebär att
grundavdraget kan vara definierat för t.ex. <code>16000-17000</code>, och <code>17100-18100</code> utan att säga något
om vad grundavdraget borde vara för <code>17050:-</code>. Det är otydligt huruvida beräkningen borde ske med det högre
eller lägre grundavdraget, det högre väljs här utan vidare motivering.</p>
<h4>Slut</h4>
<p>Slutinkomst för att beräkna skattesumman, det är månatlig inkomst.</p>
<h4>Procentgraf</h4>
<p>En graf som visar procent skatt över krona tjänad, olika start och slut-löner, samt steg kan väljas.</p>
<p>Ytterligare kan vad som visas i grafen väljas genom att klicka i rutorna.</p>
<h4>Procentgraf lutning</h4>
<p>En beräkning av lutningen mellan punker av den procentuella skatten för lönespannet.</p>
<p>Det är bara <code>(y2 - y1) / (x2 - x1)</code>, det vill säga den procentuella skillnaden mellan två löner, genom
skillnaden i kronor mella samma två löner.</p>
<p>Det är ekvivalent med <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/sabeskattasdinlon/marginalskatt.4.61589f801118cb2b7b280006375.html">marginalskatten</a>,
fast i en svåravläslig form, se 'Absolutgraf lutning' för marginalskatten som den brukar visas.</p>
<h4>Absolutgraf</h4>
<p>En graf som visar skatt i kronor över krona tjänad, olika start och slut-löner, samt steg kan väljas.</p>
<p>Ytterligare kan vad som visas i grafen väljas genom att klicka i rutorna.</p>
<h4>Absolutgraf lutning</h4>
<p>En beräkning av lutningen mellan punker av den absoluta skatten för lönespannet.</p>
<p>Det är bara <code>(y2 - y1) / (x2 - x1)</code>, det vill säga den absoluta skillnaden i kronor mellan två löner, genom
skillnaden i kronor mella samma två löner.</p>
<p>Det är ekvivalent med <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/sabeskattasdinlon/marginalskatt.4.61589f801118cb2b7b280006375.html">marginalskatten</a>.</p>
<h3>Cirkeldiagram</h3>
<p>Cirkeldiagrammet visar skattefördelningen. Vid lägre löner är det bara arbetsgivaravgift i pajen, men
när lönen ökar kommer andra skatter, så som kommunal- och till slut statlig inkomstskatt ta upp delar av den.</p>
<h3>Tabell</h3>
<p>En tabell som visar de olika posterna som utgör skatten i procent och kronor.</p>
"#;

pub const GRUNDAVDRAG: &str = r#"<h3>Grundavdrag</h3>
<p>Grundavdraget dras från inkomsterna från gällande år, i regel går skatten på det återstående beloppet.</p>
<p>Skiljer sig beroende på inkomst och ålder.</p>
<p>Gällande grundavdrag hämtas från Skatteverkets API
<a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Grundavdrag%20f%C3%B6r%20personer%20under%2066%20%C3%A5r">här för de under 66</a>,
och <a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Grundavdrag%20f%C3%B6r%20personer%2066%20%C3%A5r%20och%20%C3%A4ldre">här för de 66 och äldre</a>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html">här</a></p>
"#;

pub const JOBBSKATTEAVDRAG: &str = r#"<h3>Jobbskatteavdrag</h3>
<p>Jobbskatteavdraget är ett avdrag som vissa inkomsttagare får göra på sin skatt,
och det mest komplicerade att räkna ut.</p>
<p>Se <a href="https://www4.skatteverket.se/rattsligvagledning/edition/2024.3/2940.html">denna länk</a>,
alternativt <a href="https://www4.skatteverket.se/rattsligvagledning/27071.html?date=2024-06-13">denna länk</a> för detaljerna.</p>
"#;

pub const KOMMUN: &str = r#"<h3>Kommun</h3>
<p>Kommun behöver anges då de kommunala skattesatserna ska kunna hämtas från skatteverkets api.</p>
<p>Apiet kan hittas <a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun">här</a>.</p>
"#;

pub const KOMMUNAL_SKATT: &str = r#"<h3>Kommunal skatt</h3>
<p>De kommunala skatterna består av fyra delar, totalen beräknas genom summering.</p>
<p>Datat för alla fyra delar kommer från
<a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun">skatteverkets api</a>.</p>
<h4>Kyrkoavgift</h4>
<p>Om inkomsttagaren är medlem i en församling betalar denne kyrkskatt, den varierar per församling.</p>
<p>Den ligger ofta kring <code>1%</code>.</p>
<p>Läs på mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/kyrkoavgift.4.3152d9ac158968eb8fd2db3.html">här</a>.</p>
<h4>Begravningsavgift</h4>
<p>Begravningsavgift betalas av alla som är folkbokförda i Sverige.</p>
<p>Läs på mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/begravningsavgift.4.1ee2ea81054cf37b1c800030.html">här</a></p>
<h4>Landstingsskatt/Regionsskatt</h4>
<p>Pengar som går till regionen.</p>
<p>Läs mer <a href="https://www.skatteverket.se/offentligaaktorer/utbetalningavskatt.4.361dc8c15312eff6fd1a79d.html">här</a></p>
<h4>Kommunalskatt</h4>
<p>Pengar som går till kommunen.</p>
<p>Läs mer <a href="https://www.skatteverket.se/offentligaaktorer/utbetalningavskatt.4.361dc8c15312eff6fd1a79d.html">här</a>(samma som ovan)</p>
"#;

pub const KOMMUNALSKATT: &str = r#"<h4>Kommunalskatt</h4>
<p>Pengar som går till kommunen.</p>
<p>Läs mer <a href="https://www.skatteverket.se/offentligaaktorer/utbetalningavskatt.4.361dc8c15312eff6fd1a79d.html">här</a>(samma som ovan)</p>
"#;

pub const KVAR_EFTER_SKATT: &str = "<h3>Kvar efter skatt</h3>
<p>Pengarna som arbetstagaren får ut efter skatt, <code>bruttolön - personliga skatter</code>.</p>
";

pub const KYRKOAVGIFT: &str = r#"<h4>Kyrkoavgift</h4>
<p>Om inkomsttagaren är medlem i en församling betalar denne kyrkskatt, den varierar per församling.</p>
<p>Den ligger ofta kring <code>1%</code>.</p>
<p>Läs på mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/kyrkoavgift.4.3152d9ac158968eb8fd2db3.html">här</a>.</p>
"#;

pub const LANDSTINGSSKATT_REGIONSSKATT: &str = r#"<h4>Landstingsskatt/Regionsskatt</h4>
<p>Pengar som går till regionen.</p>
<p>Läs mer <a href="https://www.skatteverket.se/offentligaaktorer/utbetalningavskatt.4.361dc8c15312eff6fd1a79d.html">här</a></p>
"#;

pub const LINJEDIAGRAM: &str = r#"<h3>Linjediagram</h3>
<p>Fyra av visualiseringarna är linjediagram.</p>
<h4>Start</h4>
<p>Välj start-inkomst att börja beräkna skatt från, det är månatlig inkomst.</p>
<h4>Steg</h4>
<p>Välj steg mellan beräkningar. För t.ex. <code>100</code> räknas skatten ut för en månatlig inkomst på
<code>100, 200, 300, ...</code> SEK. Lägre steg gör beräkningen tyngre (tar längre tid), går stegen under 100
blir även beräkningen något tvivelaktig.</p>
<p>Skatteverkets grundavdrag ges som spann med <code>100:-</code>-glipor mellan. Det innebär att
grundavdraget kan vara definierat för t.ex. <code>16000-17000</code>, och <code>17100-18100</code> utan att säga något
om vad grundavdraget borde vara för <code>17050:-</code>. Det är otydligt huruvida beräkningen borde ske med det högre
eller lägre grundavdraget, det högre väljs här utan vidare motivering.</p>
<h4>Slut</h4>
<p>Slutinkomst för att beräkna skattesumman, det är månatlig inkomst.</p>
<h4>Procentgraf</h4>
<p>En graf som visar procent skatt över krona tjänad, olika start och slut-löner, samt steg kan väljas.</p>
<p>Ytterligare kan vad som visas i grafen väljas genom att klicka i rutorna.</p>
<h4>Procentgraf lutning</h4>
<p>En beräkning av lutningen mellan punker av den procentuella skatten för lönespannet.</p>
<p>Det är bara <code>(y2 - y1) / (x2 - x1)</code>, det vill säga den procentuella skillnaden mellan två löner, genom
skillnaden i kronor mella samma två löner.</p>
<p>Det är ekvivalent med <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/sabeskattasdinlon/marginalskatt.4.61589f801118cb2b7b280006375.html">marginalskatten</a>,
fast i en svåravläslig form, se 'Absolutgraf lutning' för marginalskatten som den brukar visas.</p>
<h4>Absolutgraf</h4>
<p>En graf som visar skatt i kronor över krona tjänad, olika start och slut-löner, samt steg kan väljas.</p>
<p>Ytterligare kan vad som visas i grafen väljas genom att klicka i rutorna.</p>
<h4>Absolutgraf lutning</h4>
<p>En beräkning av lutningen mellan punker av den absoluta skatten för lönespannet.</p>
<p>Det är bara <code>(y2 - y1) / (x2 - x1)</code>, det vill säga den absoluta skillnaden i kronor mellan två löner, genom
skillnaden i kronor mella samma två löner.</p>
<p>Det är ekvivalent med <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/sabeskattasdinlon/marginalskatt.4.61589f801118cb2b7b280006375.html">marginalskatten</a>.</p>
"#;

pub const MEDLEM_I_SVENSKA_KYRKAN: &str = r#"<h3>Medlem i Svenska kyrkan</h3>
<p>Om man är medlem i Svenska kyrkan betalar man i regel kyrkoavgift. Om rutan för
medlemskap kryssas i måste även rätt församling väljas för skatteberäkningen ska bli korrekt.</p>
<p>Läs mer om kyrkoavgiften <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/kyrkoavgift.4.3152d9ac158968eb8fd2db3.html">här</a>.</p>
<p>Datat för skattesats per församling hämtas från <a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun">skatteverkets api</a>.</p>
"#;

pub const PROCENTGRAF: &str = "<h4>Procentgraf</h4>
<p>En graf som visar procent skatt över krona tjänad, olika start och slut-löner, samt steg kan väljas.</p>
<p>Ytterligare kan vad som visas i grafen väljas genom att klicka i rutorna.</p>
";

pub const PROCENTGRAF_LUTNING: &str = r#"<h4>Procentgraf lutning</h4>
<p>En beräkning av lutningen mellan punker av den procentuella skatten för lönespannet.</p>
<p>Det är bara <code>(y2 - y1) / (x2 - x1)</code>, det vill säga den procentuella skillnaden mellan två löner, genom
skillnaden i kronor mella samma två löner.</p>
<p>Det är ekvivalent med <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/sabeskattasdinlon/marginalskatt.4.61589f801118cb2b7b280006375.html">marginalskatten</a>,
fast i en svåravläslig form, se 'Absolutgraf lutning' för marginalskatten som den brukar visas.</p>
"#;

pub const PUBLIC_SERVICE_AVGIFT: &str = r#"<h3>Public service-avgift</h3>
<p>Public service-avgiften betalas av alla som tjänar över <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html">grundavdraget</a>.</p>
<p>Tjänar man <code>121 919:-</code> eller lägre betalar man 1% av inkomsten, annars betalar man maxgränsen <code>1219:-</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/publicserviceavgift.4.22501d9e166a8cb399f31dd.html">här</a></p>
"#;

pub const SKATTEBERAKNING: &str = r#"<h1>Skatteberäkning</h1>
<p>En enkel webb-applikation för att visa skatt</p>
<h2>Syfte</h2>
<p>Att visa vilken skatt man betalar vid en given lön, och hur den skatten i sig är uppdelad.</p>
<h2>Bakgrund</h2>
<p>Idag är det inte helt enkelt att räkna ut vilka skatter som gäller vid vilka inkomster,
man kan använda <a href="https://www.ekonomifakta.se/">Ekonomifakta</a>, men då de har en stark politisk agenda
(de är en arbetsgivarorganisation) är det svårt att lita på information som sprids därifrån.</p>
<p>Därav denna applikation, en eventuell politisk agenda spelar ingen roll, koden
är öppen och ni kan dubbelkolla alla beräkningar om ni så önskar. Hittas ett fel så informera gärna,
skapa en issue här på Github så korrigeras det.</p>
<h2>Arkitektur</h2>
<p>Appen är skriven <a href="https://github.com/yewstack/yew">yew</a> och blir där med en <a href="https://webassembly.org/">wasm-modul</a>,
utöver den så är det bara ren <a href="https://en.wikipedia.org/wiki/HTML">html</a>, det är därför sidan är så
estetiskt otilltalande.</p>
<h3>Uppdelning</h3>
<p>Appen består av tre delar, ett lib: <a href="./skatt-lib">skatt-lib</a>, själva applikationskoden <a href="./skatt-fe">skatt-fe</a>,
och en liten app som bara genererar en lista av sveriges kommuner <a href="./offline-runner">offline-runner</a>.</p>
<h2>Data</h2>
<p>Det är inte helt enkelt att förstå hur skatten ska beräknas, den här applikationens värden skiljer sig något
mot Ekonomifaktas.<br />
Specifikt på hur jobbskatteavdrag räknas ut för inkomster under ca <code>10000:-</code> per månad. Varför det skiljer sig
är ännu otydligt, någon räknar fel, mer på beräkningar senare. I fallet av jobbskatteavdrag
så räknar denna applikation ut ett högre avdrag än Ekonomifakta på låga inkomster.</p>
<p>När inkomsterna överstiger <code>10000:-</code> per månad blir beräkningarna närmast identiska.</p>
<h3>Ålder</h3>
<p>Ålder behöver anges endast för att ta reda på huruvida inkomsttagaren fyllt 66 år vid årets ingång,
vissa skatteregler skiljer sig för dessa.</p>
<h3>Kommun</h3>
<p>Kommun behöver anges då de kommunala skattesatserna ska kunna hämtas från skatteverkets api.</p>
<p>Apiet kan hittas <a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun">här</a>.</p>
<h3>Medlem i Svenska kyrkan</h3>
<p>Om man är medlem i Svenska kyrkan betalar man i regel kyrkoavgift. Om rutan för
medlemskap kryssas i måste även rätt församling väljas för skatteberäkningen ska bli korrekt.</p>
<p>Läs mer om kyrkoavgiften <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/kyrkoavgift.4.3152d9ac158968eb8fd2db3.html">här</a>.</p>
<p>Datat för skattesats per församling hämtas från <a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun">skatteverkets api</a>.</p>
<h2>Grafer</h2>
<p>I skrivande stund kan datat visualiseras på sex olika sätt.</p>
<h3>Linjediagram</h3>
<p>Fyra av visualiseringarna är linjediagram.</p>
<h4>Start</h4>
<p>Välj start-inkomst att börja beräkna skatt från, det är månatlig inkomst.</p>
<h4>Steg</h4>
<p>Välj steg mellan beräkningar. För t.ex. <code>100</code> räknas skatten ut för en månatlig inkomst på
<code>100, 200, 300, ...</code> SEK. Lägre steg gör beräkningen tyngre (tar längre tid), går stegen under 100
blir även beräkningen något tvivelaktig.</p>
<p>Skatteverkets grundavdrag ges som spann med <code>100:-</code>-glipor mellan. Det innebär att
grundavdraget kan vara definierat för t.ex. <code>16000-17000</code>, och <code>17100-18100</code> utan att säga något
om vad grundavdraget borde vara för <code>17050:-</code>. Det är otydligt huruvida beräkningen borde ske med det högre
eller lägre grundavdraget, det högre väljs här utan vidare motivering.</p>
<h4>Slut</h4>
<p>Slutinkomst för att beräkna skattesumman, det är månatlig inkomst.</p>
<h4>Procentgraf</h4>
<p>En graf som visar procent skatt över krona tjänad, olika start och slut-löner, samt steg kan väljas.</p>
<p>Ytterligare kan vad som visas i grafen väljas genom att klicka i rutorna.</p>
<h4>Procentgraf lutning</h4>
<p>En beräkning av lutningen mellan punker av den procentuella skatten för lönespannet.</p>
<p>Det är bara <code>(y2 - y1) / (x2 - x1)</code>, det vill säga den procentuella skillnaden mellan två löner, genom
skillnaden i kronor mella samma två löner.</p>
<p>Det är ekvivalent med <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/sabeskattasdinlon/marginalskatt.4.61589f801118cb2b7b280006375.html">marginalskatten</a>,
fast i en svåravläslig form, se 'Absolutgraf lutning' för marginalskatten som den brukar visas.</p>
<h4>Absolutgraf</h4>
<p>En graf som visar skatt i kronor över krona tjänad, olika start och slut-löner, samt steg kan väljas.</p>
<p>Ytterligare kan vad som visas i grafen väljas genom att klicka i rutorna.</p>
<h4>Absolutgraf lutning</h4>
<p>En beräkning av lutningen mellan punker av den absoluta skatten för lönespannet.</p>
<p>Det är bara <code>(y2 - y1) / (x2 - x1)</code>, det vill säga den absoluta skillnaden i kronor mellan två löner, genom
skillnaden i kronor mella samma två löner.</p>
<p>Det är ekvivalent med <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/sabeskattasdinlon/marginalskatt.4.61589f801118cb2b7b280006375.html">marginalskatten</a>.</p>
<h3>Cirkeldiagram</h3>
<p>Cirkeldiagrammet visar skattefördelningen. Vid lägre löner är det bara arbetsgivaravgift i pajen, men
när lönen ökar kommer andra skatter, så som kommunal- och till slut statlig inkomstskatt ta upp delar av den.</p>
<h3>Tabell</h3>
<p>En tabell som visar de olika posterna som utgör skatten i procent och kronor.</p>
<h2>Beräkningar</h2>
<p>Skatten består av ett antal delar.</p>
<h3>Kommunal skatt</h3>
<p>De kommunala skatterna består av fyra delar, totalen beräknas genom summering.</p>
<p>Datat för alla fyra delar kommer från
<a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun">skatteverkets api</a>.</p>
<h4>Kyrkoavgift</h4>
<p>Om inkomsttagaren är medlem i en församling betalar denne kyrkskatt, den varierar per församling.</p>
<p>Den ligger ofta kring <code>1%</code>.</p>
<p>Läs på mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/kyrkoavgift.4.3152d9ac158968eb8fd2db3.html">här</a>.</p>
<h4>Begravningsavgift</h4>
<p>Begravningsavgift betalas av alla som är folkbokförda i Sverige.</p>
<p>Läs på mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/begravningsavgift.4.1ee2ea81054cf37b1c800030.html">här</a></p>
<h4>Landstingsskatt/Regionsskatt</h4>
<p>Pengar som går till regionen.</p>
<p>Läs mer <a href="https://www.skatteverket.se/offentligaaktorer/utbetalningavskatt.4.361dc8c15312eff6fd1a79d.html">här</a></p>
<h4>Kommunalskatt</h4>
<p>Pengar som går till kommunen.</p>
<p>Läs mer <a href="https://www.skatteverket.se/offentligaaktorer/utbetalningavskatt.4.361dc8c15312eff6fd1a79d.html">här</a>(samma som ovan)</p>
<h3>Statlig inkomstskatt</h3>
<p><a href="https://www.skatteverket.se/privat/etjansterochblanketter/svarpavanligafragor/inkomstavtjanst/privattjansteinkomsterfaq/narskamanbetalastatliginkomstskattochhurhogarden.5.10010ec103545f243e8000166.html">Statlig inkomstskatt</a>
betalar man om man tjänar en bit över den så kallade skiktgränsen.</p>
<p>Men, eftersom <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html">grundavdraget</a> går först börjar man inte betala statlig inkomstskatt förrän man
tjänar en summa som överstiger <code>skiktgräns - grundavdrag</code>.</p>
<p>Beräknas genom att ta <code>20%</code> av lönen som överstiger <code>skiktgräns - grundavdrag</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/etjansterochblanketter/svarpavanligafragor/inkomstavtjanst/privattjansteinkomsterfaq/narskamanbetalastatliginkomstskattochhurhogarden.5.10010ec103545f243e8000166.html">här</a></p>
<h3>Public service-avgift</h3>
<p>Public service-avgiften betalas av alla som tjänar över <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html">grundavdraget</a>.</p>
<p>Tjänar man <code>121 919:-</code> eller lägre betalar man 1% av inkomsten, annars betalar man maxgränsen <code>1219:-</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/publicserviceavgift.4.22501d9e166a8cb399f31dd.html">här</a></p>
<h3>Grundavdrag</h3>
<p>Grundavdraget dras från inkomsterna från gällande år, i regel går skatten på det återstående beloppet.</p>
<p>Skiljer sig beroende på inkomst och ålder.</p>
<p>Gällande grundavdrag hämtas från Skatteverkets API
<a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Grundavdrag%20f%C3%B6r%20personer%20under%2066%20%C3%A5r">här för de under 66</a>,
och <a href="https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Grundavdrag%20f%C3%B6r%20personer%2066%20%C3%A5r%20och%20%C3%A4ldre">här för de 66 och äldre</a>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html">här</a></p>
<h3>Jobbskatteavdrag</h3>
<p>Jobbskatteavdraget är ett avdrag som vissa inkomsttagare får göra på sin skatt,
och det mest komplicerade att räkna ut.</p>
<p>Se <a href="https://www4.skatteverket.se/rattsligvagledning/edition/2024.3/2940.html">denna länk</a>,
alternativt <a href="https://www4.skatteverket.se/rattsligvagledning/27071.html?date=2024-06-13">denna länk</a> för detaljerna.</p>
<h3>Skattereduktion förvärvsinkomst</h3>
<p>Om man tjänar mer än <code>40 000:-</code> om året kan man få en skattereduktion upp till <code>1500:-</code>.</p>
<p>Beräknas genom att undersöka inkomsten:</p>
<p>Är den mellan <code>40 000:-</code> och <code>240 000:-</code> får inkomsttagaren <code>0.75%</code> av inkomsten i skattereduktion.</p>
<p>Överstiger den <code>240 000:-</code> får inkomsttagaren <code>1500:-</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/skattereduktioner.4.3810a01c150939e893f1a17e.html">här</a></p>
<h3>Total personlig skatt</h3>
<p>Summan av alla skatter som betalas av inkomsttagaren själv, det är alla skatter och avdrag exclusive
arbetsgivaravgiften.</p>
<h3>Arbetsgivaravgift</h3>
<p>Arbetsgivaravgiften betraktas av vissa som skatt och tas därmed med här, den betalas av arbetsgivaren
och baseras på bruttolönen.</p>
<p>För personer som vid årets ingång into fyllt 66 år är det <code>31.24%</code>, för personer
som inte fyllt 66 år vid årets ingång är det <code>10.21%</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/foretag/arbetsgivare/arbetsgivaravgifterochskatteavdrag/arbetsgivaravgifter.4.233f91f71260075abe8800020817.html">här</a></p>
<h3>Total skattesumma</h3>
<p>Summan av alla skatter som betalas av inkomsttagaren själv, alla skatter och avdrag, inklusive
arbetsgivaravgiften.</p>
<h3>Kvar efter skatt</h3>
<p>Pengarna som arbetstagaren får ut efter skatt, <code>bruttolön - personliga skatter</code>.</p>
"#;

pub const SKATTEREDUKTION_FORVARVSINKOMST: &str = r#"<h3>Skattereduktion förvärvsinkomst</h3>
<p>Om man tjänar mer än <code>40 000:-</code> om året kan man få en skattereduktion upp till <code>1500:-</code>.</p>
<p>Beräknas genom att undersöka inkomsten:</p>
<p>Är den mellan <code>40 000:-</code> och <code>240 000:-</code> får inkomsttagaren <code>0.75%</code> av inkomsten i skattereduktion.</p>
<p>Överstiger den <code>240 000:-</code> får inkomsttagaren <code>1500:-</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/skattereduktioner.4.3810a01c150939e893f1a17e.html">här</a></p>
"#;

pub const SLUT: &str = "<h4>Slut</h4>
<p>Slutinkomst för att beräkna skattesumman, det är månatlig inkomst.</p>
";

pub const START: &str = "<h4>Start</h4>
<p>Välj start-inkomst att börja beräkna skatt från, det är månatlig inkomst.</p>
";

pub const STATLIG_INKOMSTSKATT: &str = r#"<h3>Statlig inkomstskatt</h3>
<p><a href="https://www.skatteverket.se/privat/etjansterochblanketter/svarpavanligafragor/inkomstavtjanst/privattjansteinkomsterfaq/narskamanbetalastatliginkomstskattochhurhogarden.5.10010ec103545f243e8000166.html">Statlig inkomstskatt</a>
betalar man om man tjänar en bit över den så kallade skiktgränsen.</p>
<p>Men, eftersom <a href="https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/grundavdrag.4.6d02084411db6e252fe80009078.html">grundavdraget</a> går först börjar man inte betala statlig inkomstskatt förrän man
tjänar en summa som överstiger <code>skiktgräns - grundavdrag</code>.</p>
<p>Beräknas genom att ta <code>20%</code> av lönen som överstiger <code>skiktgräns - grundavdrag</code>.</p>
<p>Läs mer <a href="https://www.skatteverket.se/privat/etjansterochblanketter/svarpavanligafragor/inkomstavtjanst/privattjansteinkomsterfaq/narskamanbetalastatliginkomstskattochhurhogarden.5.10010ec103545f243e8000166.html">här</a></p>
"#;

pub const STEG: &str = "<h4>Steg</h4>
<p>Välj steg mellan beräkningar. För t.ex. <code>100</code> räknas skatten ut för en månatlig inkomst på
<code>100, 200, 300, ...</code> SEK. Lägre steg gör beräkningen tyngre (tar längre tid), går stegen under 100
blir även beräkningen något tvivelaktig.</p>
<p>Skatteverkets grundavdrag ges som spann med <code>100:-</code>-glipor mellan. Det innebär att
grundavdraget kan vara definierat för t.ex. <code>16000-17000</code>, och <code>17100-18100</code> utan att säga något
om vad grundavdraget borde vara för <code>17050:-</code>. Det är otydligt huruvida beräkningen borde ske med det högre
eller lägre grundavdraget, det högre väljs här utan vidare motivering.</p>
";

pub const SYFTE: &str = "<h2>Syfte</h2>
<p>Att visa vilken skatt man betalar vid en given lön, och hur den skatten i sig är uppdelad.</p>
";

pub const TABELL: &str = "<h3>Tabell</h3>
<p>En tabell som visar de olika posterna som utgör skatten i procent och kronor.</p>
";

pub const TOTAL_PERSONLIG_SKATT: &str = "<h3>Total personlig skatt</h3>
<p>Summan av alla skatter som betalas av inkomsttagaren själv, det är alla skatter och avdrag exclusive
arbetsgivaravgiften.</p>
";

pub const TOTAL_SKATTESUMMA: &str = "<h3>Total skattesumma</h3>
<p>Summan av alla skatter som betalas av inkomsttagaren själv, alla skatter och avdrag, inklusive
arbetsgivaravgiften.</p>
";

pub const UPPDELNING: &str = r#"<h3>Uppdelning</h3>
<p>Appen består av tre delar, ett lib: <a href="./skatt-lib">skatt-lib</a>, själva applikationskoden <a href="./skatt-fe">skatt-fe</a>,
och en liten app som bara genererar en lista av sveriges kommuner <a href="./offline-runner">offline-runner</a>.</p>
"#;

pub const ALDER: &str = "<h3>Ålder</h3>
<p>Ålder behöver anges endast för att ta reda på huruvida inkomsttagaren fyllt 66 år vid årets ingång,
vissa skatteregler skiljer sig för dessa.</p>
";
